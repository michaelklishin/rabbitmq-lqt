import { useState, useEffect, useCallback } from 'react'
import { validateQuery, initWasm, isWasmAvailable, getAutocompleteData } from '../wasm/qlValidator'
import type { ValidationResult, AutocompleteData } from '../wasm/types'

interface QLPanelProps {
  query: string
  limit: number
  onQueryChange: (query: string) => void
  onLimitChange: (limit: number) => void
  onRunQuery: () => void
  isLoading: boolean
}

const EXAMPLE_QUERIES = [
  { query: ':errors', description: 'All error logs' },
  { query: '@24h subsystem == "connections"', description: 'Connection logs, last 24h' },
  { query: ':crashes | sort timestamp desc', description: 'Crashes preset, newest first' },
  { query: 'labels any ["tls", "disconnects"]', description: 'TLS or disconnect events' },
  { query: 'message contains "timeout" | limit 50', description: 'Messages with "timeout"' },
  { query: '@1h severity == "warning" or severity == "error"', description: 'Warnings/errors last hour' },
  { query: 'subsystem == "queues" | sort timestamp desc | limit 200', description: 'Recent queue logs' },
  { query: ':errors_or_crashes @7d', description: 'Errors or crashes in the last week' },
]

export function QLPanel({
  query,
  limit,
  onQueryChange,
  onLimitChange,
  onRunQuery,
  isLoading,
}: QLPanelProps) {
  const [validation, setValidation] = useState<ValidationResult | null>(null)
  const [wasmReady, setWasmReady] = useState(false)
  const [showExamples, setShowExamples] = useState(false)
  const [showReference, setShowReference] = useState(false)
  const [showHelp, setShowHelp] = useState(false)
  const [autocompleteData, setAutocompleteData] = useState<AutocompleteData | null>(null)

  useEffect(() => {
    initWasm().then(() => {
      setWasmReady(isWasmAvailable())
      if (isWasmAvailable()) {
        setAutocompleteData(getAutocompleteData())
      }
    })
  }, [])

  useEffect(() => {
    if (wasmReady && query.trim()) {
      const result = validateQuery(query)
      setValidation(result)
    } else if (!query.trim()) {
      setValidation(null)
    }
  }, [query, wasmReady])

  const isValid = validation?.valid ?? true
  const canRun = query.trim().length > 0 && (isValid || !wasmReady) && !isLoading

  const handleKeyDown = useCallback((e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') {
      e.preventDefault()
      if (canRun) {
        onRunQuery()
      }
    }
  }, [canRun, onRunQuery])

  const handleExampleClick = (exampleQuery: string) => {
    onQueryChange(exampleQuery)
    setShowExamples(false)
  }

  return (
    <div className="bg-white shadow-sm border border-gray-200 rounded-lg p-4 space-y-4">
      {/* Main query input row */}
      <div className="flex gap-3 items-start">
        <div className="flex-1">
          <div className="relative">
            <input
              type="text"
              value={query}
              onChange={(e) => onQueryChange(e.target.value)}
              onKeyDown={handleKeyDown}
              placeholder=':errors | limit 100'
              className={`w-full px-4 py-3 font-mono text-sm border rounded-lg focus:outline-none focus:ring-2 ${
                query.trim() && validation && !validation.valid
                  ? 'border-red-500 bg-red-50 focus:ring-red-500'
                  : query.trim() && validation?.valid
                  ? 'border-green-500 bg-green-50 focus:ring-green-500'
                  : 'border-gray-300 focus:ring-blue-500'
              }`}
            />
            {query.trim() && validation && (
              <div className="absolute right-3 top-1/2 -translate-y-1/2">
                {validation.valid ? (
                  <svg className="w-5 h-5 text-green-500" fill="currentColor" viewBox="0 0 20 20">
                    <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clipRule="evenodd" />
                  </svg>
                ) : (
                  <svg className="w-5 h-5 text-red-500" fill="currentColor" viewBox="0 0 20 20">
                    <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clipRule="evenodd" />
                  </svg>
                )}
              </div>
            )}
          </div>
        </div>

        <select
          value={limit}
          onChange={(e) => onLimitChange(parseInt(e.target.value))}
          className="px-3 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm"
          title="Result limit"
        >
          <option value={500}>500</option>
          <option value={1000}>1000</option>
          <option value={3000}>3000</option>
          <option value={5000}>5000</option>
          <option value={10000}>10000</option>
          <option value={20000}>20000</option>
          <option value={50000}>50000</option>
        </select>

        <button
          onClick={onRunQuery}
          disabled={!canRun}
          className={`px-6 py-3 rounded-lg font-medium text-white transition-colors ${
            canRun
              ? 'bg-blue-600 hover:bg-blue-700'
              : 'bg-gray-400 cursor-not-allowed'
          }`}
        >
          {isLoading ? 'Running...' : 'Run'}
        </button>
      </div>

      {/* Controls row */}
      <div className="flex items-center justify-between text-sm">
        <div className="flex items-center gap-4">
          <button
            onClick={() => setShowHelp(!showHelp)}
            className="text-blue-600 hover:text-blue-800 font-medium"
          >
            {showHelp ? 'Hide Help' : 'Help'}
          </button>
          <button
            onClick={() => setShowExamples(!showExamples)}
            className="text-blue-600 hover:text-blue-800 font-medium"
          >
            {showExamples ? 'Hide Examples' : 'Examples'}
          </button>
          <button
            onClick={() => setShowReference(!showReference)}
            className="text-blue-600 hover:text-blue-800 font-medium"
          >
            {showReference ? 'Hide Reference' : 'Quick Reference'}
          </button>
          {autocompleteData && (
            <div className="flex items-center gap-2 text-gray-500">
              <span>Presets:</span>
              {autocompleteData.presets.slice(0, 5).map((preset) => (
                <button
                  key={preset.name}
                  onClick={() => handleExampleClick(`:${preset.name}`)}
                  className="px-2 py-0.5 text-xs font-mono bg-gray-100 text-gray-700 rounded hover:bg-gray-200"
                  title={preset.description}
                >
                  :{preset.name}
                </button>
              ))}
            </div>
          )}
        </div>

        <div className="text-gray-500">
          {!wasmReady && (
            <span className="text-yellow-600">
              <svg className="w-4 h-4 inline-block mr-1" fill="currentColor" viewBox="0 0 20 20">
                <path fillRule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clipRule="evenodd" />
              </svg>
              Server-side validation only
            </span>
          )}
        </div>
      </div>

      {/* Error message */}
      {query.trim() && validation && !validation.valid && (
        <div className="bg-red-50 border border-red-200 rounded-lg p-3">
          <div className="flex items-start gap-2">
            <div className="text-sm text-red-700">
              <span className="font-medium">Invalid query:</span> {validation.error_message}
              {validation.suggestions.length > 0 && (
                <span className="ml-2">
                  Did you mean: <code className="font-mono bg-red-100 px-1 rounded">{validation.suggestions[0]}</code>?
                </span>
              )}
            </div>
          </div>
        </div>
      )}

      {/* Help panel */}
      {showHelp && (
        <div className="bg-blue-50 border border-blue-200 rounded-lg p-4 space-y-4">
          {/* Getting Started */}
          <div>
            <h3 className="text-sm font-semibold text-blue-800 mb-2">Getting Started</h3>
            <div className="text-sm text-gray-700 space-y-2">
              <p>
                Type a query in the input field above, then press <kbd className="px-1.5 py-0.5 bg-gray-200 rounded text-xs font-mono">Enter</kbd> or
                click <span className="font-medium">Run</span> to search your logs.
              </p>
              <p>
                <span className="font-medium">Simplest query:</span> Just type <code className="bg-blue-100 px-1 rounded">:errors</code> and
                press Enter to see all error logs. That's it!
              </p>
            </div>
          </div>

          {/* Building Blocks */}
          <div>
            <h3 className="text-sm font-semibold text-blue-800 mb-2">Query Building Blocks</h3>
            <div className="text-sm text-gray-700 space-y-1.5">
              <p>
                <span className="font-medium text-blue-700">1. Presets</span> start with <code className="bg-blue-100 px-1 rounded">:</code> —
                ready-made filters like <code className="bg-blue-100 px-1 rounded">:errors</code>,{' '}
                <code className="bg-blue-100 px-1 rounded">:crashes</code>, or{' '}
                <code className="bg-blue-100 px-1 rounded">:errors_or_crashes</code>
              </p>
              <p>
                <span className="font-medium text-blue-700">2. Time ranges</span> start with <code className="bg-blue-100 px-1 rounded">@</code> —
                like <code className="bg-blue-100 px-1 rounded">@1h</code> (last hour),{' '}
                <code className="bg-blue-100 px-1 rounded">@24h</code> (last day), or{' '}
                <code className="bg-blue-100 px-1 rounded">@7d</code> (last week)
              </p>
              <p>
                <span className="font-medium text-blue-700">3. Filters</span> match specific fields —
                like <code className="bg-blue-100 px-1 rounded">severity == "error"</code> or{' '}
                <code className="bg-blue-100 px-1 rounded">message contains "timeout"</code>
              </p>
              <p>
                <span className="font-medium text-blue-700">4. Pipeline</span> stages use <code className="bg-blue-100 px-1 rounded">|</code> —
                like <code className="bg-blue-100 px-1 rounded">| limit 100</code> or{' '}
                <code className="bg-blue-100 px-1 rounded">| sort timestamp desc</code>
              </p>
            </div>
          </div>

          {/* Combining */}
          <div>
            <h3 className="text-sm font-semibold text-blue-800 mb-2">Combining Conditions</h3>
            <div className="text-sm text-gray-700 space-y-1.5">
              <p>
                <span className="font-medium text-blue-700">Chain building blocks:</span>{' '}
                <code className="bg-blue-100 px-1 rounded">@24h :crashes | limit 50</code> — crashes from last 24h, limit to 50
              </p>
              <p>
                <span className="font-medium text-blue-700">AND:</span>{' '}
                <code className="bg-blue-100 px-1 rounded">severity == "error" and subsystem == "connections"</code> — both must match
              </p>
              <p>
                <span className="font-medium text-blue-700">OR:</span>{' '}
                <code className="bg-blue-100 px-1 rounded">severity == "error" or severity == "warning"</code> — either matches
              </p>
              <p>
                <span className="font-medium text-blue-700">NOT:</span>{' '}
                <code className="bg-blue-100 px-1 rounded">not severity == "debug"</code> — excludes debug logs
              </p>
              <p>
                <span className="font-medium text-blue-700">Grouping:</span>{' '}
                <code className="bg-blue-100 px-1 rounded">(severity == "error" or severity == "warning") and message contains "timeout"</code>
              </p>
              <p>
                <span className="font-medium text-blue-700">Labels:</span>{' '}
                <code className="bg-blue-100 px-1 rounded">labels any ["tls", "auth"]</code> — entries with either label
              </p>
            </div>
          </div>

          <p className="text-xs text-gray-500 pt-2 border-t border-blue-200">
            Tip: Click <span className="font-medium">Examples</span> to see more queries you can try, or click on any preset button above to use it directly.
          </p>
        </div>
      )}

      {/* Examples panel */}
      {showExamples && (
        <div className="bg-gray-50 border border-gray-200 rounded-lg p-4">
          <h3 className="text-sm font-semibold text-gray-700 mb-3">Example Queries</h3>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-2">
            {EXAMPLE_QUERIES.map((example, index) => (
              <div
                key={index}
                className="flex items-center justify-between gap-3 p-2 rounded hover:bg-gray-100 cursor-pointer"
                onClick={() => handleExampleClick(example.query)}
              >
                <code className="text-sm font-mono text-blue-700 truncate">{example.query}</code>
                <span className="text-xs text-gray-500 whitespace-nowrap">{example.description}</span>
              </div>
            ))}
          </div>
          {autocompleteData && (
            <div className="mt-4 pt-4 border-t border-gray-200">
              <h4 className="text-sm font-semibold text-gray-700 mb-2">All Presets</h4>
              <div className="flex flex-wrap gap-2">
                {autocompleteData.presets.map((preset) => (
                  <button
                    key={preset.name}
                    onClick={() => handleExampleClick(`:${preset.name}`)}
                    className="px-2 py-1 text-xs font-mono bg-blue-100 text-blue-700 rounded hover:bg-blue-200"
                    title={preset.description}
                  >
                    :{preset.name}
                  </button>
                ))}
              </div>
            </div>
          )}
        </div>
      )}

      {/* Quick Reference panel */}
      {showReference && autocompleteData && (
        <div className="bg-gray-50 border border-gray-200 rounded-lg p-4">
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 text-sm text-gray-600">
            <div>
              <span className="font-medium text-gray-700">Fields:</span>{' '}
              {autocompleteData.fields.map(f => f.name).join(', ')}
            </div>
            <div>
              <span className="font-medium text-gray-700">Severity:</span>{' '}
              {autocompleteData.severities.join(', ')}
            </div>
            <div>
              <span className="font-medium text-gray-700">Operators:</span>{' '}
              {autocompleteData.operators.map(o => o.symbol).join(', ')}
            </div>
            <div>
              <span className="font-medium text-gray-700">Pipeline:</span>{' '}
              {autocompleteData.pipeline_stages.map(s => s.name).join(', ')}
            </div>
            <div>
              <span className="font-medium text-gray-700">Duration:</span>{' '}
              {autocompleteData.duration_units.map(d => d.suffix).join(', ')}
              {' '}(e.g., @24h, @7d)
            </div>
            <div>
              <span className="font-medium text-gray-700">Special:</span>{' '}
              {autocompleteData.special_filters.join(', ')}
            </div>
          </div>
        </div>
      )}
    </div>
  )
}
