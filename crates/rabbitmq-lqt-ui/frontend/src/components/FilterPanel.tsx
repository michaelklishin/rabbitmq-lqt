import { useMemo } from 'react'
import { MetadataResponse, QueryParams } from '../api/client'

interface FilterPanelProps {
  metadata: MetadataResponse | null
  filters: QueryParams
  onFilterChange: (filters: QueryParams) => void
  collapsed: boolean
  onToggleCollapse: () => void
}

export function FilterPanel({ metadata, filters, onFilterChange, collapsed, onToggleCollapse }: FilterPanelProps) {
  const updateFilter = <K extends keyof QueryParams>(key: K, value: QueryParams[K]) => {
    onFilterChange({
      ...filters,
      [key]: value,
    })
  }

  // Memoize parsed labels to avoid repeated string splitting
  const currentLabels = useMemo(
    () => filters.labels?.split(',').filter(Boolean) || [],
    [filters.labels]
  )

  const currentLabelsSet = useMemo(
    () => new Set(currentLabels),
    [currentLabels]
  )

  const handleLabelChange = (label: string, checked: boolean) => {
    const newLabels = checked
      ? [...currentLabels, label]
      : currentLabels.filter((l) => l !== label)
    updateFilter('labels', newLabels.length > 0 ? newLabels.join(',') : undefined)
  }

  const clearFilters = () => {
    onFilterChange({ limit: 1000 })
  }

  if (collapsed) {
    return (
      <div className="bg-white shadow-sm border border-gray-200 rounded-lg py-4 flex flex-col items-center">
        <button
          onClick={onToggleCollapse}
          className="p-1.5 text-gray-500 hover:text-gray-700 hover:bg-gray-100 rounded transition-colors"
          title="Expand filters"
          aria-label="Expand filters"
        >
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
          </svg>
        </button>
        <span className="mt-3 text-xs font-medium text-gray-500" style={{ writingMode: 'vertical-lr' }}>
          Filters
        </span>
      </div>
    )
  }

  return (
    <div className="bg-white shadow-sm border border-gray-200 rounded-lg p-6 space-y-6">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          <button
            onClick={onToggleCollapse}
            className="p-1 text-gray-500 hover:text-gray-700 hover:bg-gray-100 rounded transition-colors"
            title="Collapse filters"
            aria-label="Collapse filters"
          >
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
            </svg>
          </button>
          <h2 className="text-lg font-semibold text-gray-900">Filters</h2>
        </div>
        <button
          onClick={clearFilters}
          className="text-sm text-blue-600 hover:text-blue-800"
        >
          Revert to Default View
        </button>
      </div>

      <div className="space-y-6">
        <div className="border-b border-gray-200 pb-4">
          <h3 className="text-sm font-semibold text-gray-900 mb-3">Well Known Issues</h3>
          <div>
            <label className="flex items-center space-x-2">
              <input
                type="checkbox"
                checked={filters.has_resolution_or_discussion_url || false}
                onChange={(e) =>
                  updateFilter('has_resolution_or_discussion_url', e.target.checked || undefined)
                }
                className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
              />
              <span className="text-sm text-gray-700">Has a resolution or discussion URL</span>
            </label>
          </div>
        </div>

        <div className="border-b border-gray-200 pb-4">
          <h3 className="text-sm font-semibold text-gray-900 mb-3">Time Range</h3>
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Since Time
              </label>
              <input
                type="text"
                value={filters.since_time || ''}
                onChange={(e) => updateFilter('since_time', e.target.value)}
                placeholder="2025-10-27 or '2 days ago'"
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                To Time
              </label>
              <input
                type="text"
                value={filters.to_time || ''}
                onChange={(e) => updateFilter('to_time', e.target.value)}
                placeholder="2025-10-27 or 'now'"
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>
          </div>
        </div>

        <div className="border-b border-gray-200 pb-4">
          <h3 className="text-sm font-semibold text-gray-900 mb-3">Zoom In</h3>
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Node
              </label>
              <select
                value={filters.node || ''}
                onChange={(e) => updateFilter('node', e.target.value || undefined)}
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                <option value="">All</option>
                {metadata?.nodes.map((node) => (
                  <option key={node} value={node}>
                    {node}
                  </option>
                ))}
              </select>
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Subsystem
              </label>
              <select
                value={filters.subsystem || ''}
                onChange={(e) => updateFilter('subsystem', e.target.value || undefined)}
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                <option value="">All</option>
                {metadata?.subsystems.map((sub) => (
                  <option key={sub} value={sub}>
                    {sub}
                  </option>
                ))}
              </select>
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                Erlang PID
              </label>
              <div className="flex gap-2">
                <input
                  type="text"
                  value={filters.erlang_pid || ''}
                  onChange={(e) => updateFilter('erlang_pid', e.target.value || undefined)}
                  placeholder="<0.208.0>"
                  className="flex-1 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
                <button
                  onClick={() => updateFilter('erlang_pid', undefined)}
                  className="px-3 py-2 text-sm font-medium text-gray-700 bg-gray-100 border border-gray-300 rounded-md hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-blue-500"
                >
                  Clear
                </button>
              </div>
            </div>
          </div>
        </div>

        <div className="border-b border-gray-200 pb-4">
          <h3 className="text-sm font-semibold text-gray-900 mb-3">Zoom In Further</h3>
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Labels
            </label>
            <div className="space-y-2 max-h-96 overflow-y-auto">
              {metadata?.labels.map((label) => (
                <label key={label} className="flex items-center space-x-2">
                  <input
                    type="checkbox"
                    checked={currentLabelsSet.has(label)}
                    onChange={(e) => handleLabelChange(label, e.target.checked)}
                    className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                  />
                  <span className="text-sm text-gray-700">{label}</span>
                </label>
              ))}
            </div>
          </div>

          {currentLabels.length > 1 && (
            <div className="mt-4">
              <label className="flex items-center space-x-2">
                <input
                  type="checkbox"
                  checked={filters.matching_all_labels || false}
                  onChange={(e) => updateFilter('matching_all_labels', e.target.checked)}
                  className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                />
                <span className="text-sm text-gray-700">Show entries that match all selected labels (an AND query)</span>
              </label>
            </div>
          )}

          <div className="mt-4">
            <label className="block text-sm font-medium text-gray-700 mb-1">
              Severity
            </label>
            <select
              value={filters.severity || ''}
              onChange={(e) => updateFilter('severity', e.target.value || undefined)}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              <option value="">All</option>
              {metadata?.severities.map((sev) => (
                <option key={sev} value={sev}>
                  {sev}
                </option>
              ))}
            </select>
          </div>
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 mb-1">
            Log entry rows to load (maximum)
          </label>
          <select
            value={filters.limit || 1000}
            onChange={(e) => updateFilter('limit', parseInt(e.target.value))}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <option value={500}>500</option>
            <option value={1000}>1000</option>
            <option value={3000}>3000</option>
            <option value={5000}>5000</option>
            <option value={10000}>10000</option>
            <option value={20000}>20000</option>
            <option value={50000}>50000</option>
          </select>
        </div>
      </div>
    </div>
  )
}
