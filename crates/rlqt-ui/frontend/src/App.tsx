import { useState, useEffect, useCallback } from 'react'
import { useQuery } from '@tanstack/react-query'
import { FilterPanel } from './components/FilterPanel'
import { LogTable } from './components/LogTable'
import { FileMetadataTab } from './components/FileMetadataTab'
import { MetadataHeader } from './components/MetadataHeader'
import { PresetFilterPanel } from './components/PresetFilterPanel'
import { QLPanel } from './components/QLPanel'
import {
  queryLogs,
  queryLogsByPreset,
  queryLogsByQL,
  getMetadata,
  getStats,
  getFileMetadata,
  QueryParams,
  PresetQueryParams,
} from './api/client'
import { formatDate } from './utils/dateFormat'

type Tab = 'filters' | 'ql' | 'metadata' | 'preset_errors_or_crashes'

function tabFromURL(): Tab {
  const params = new URLSearchParams(window.location.search)
  const tab = params.get('tab')
  if (tab === 'ql') return 'ql'
  if (tab === 'metadata') return 'metadata'
  if (tab === 'preset_errors_or_crashes') return 'preset_errors_or_crashes'
  return 'filters'
}

interface QLState {
  query: string
  limit: number
}

function qlStateFromURL(): QLState {
  const params = new URLSearchParams(window.location.search)
  return {
    query: params.get('ql_query') || '',
    limit: parseInt(params.get('ql_limit') || '1000'),
  }
}

function filtersFromURL(): QueryParams {
  const params = new URLSearchParams(window.location.search)
  const filters: QueryParams = {
    limit: parseInt(params.get('limit') || '1000'),
  }

  if (params.has('since_time')) filters.since_time = params.get('since_time')!
  if (params.has('to_time')) filters.to_time = params.get('to_time')!
  if (params.has('severity')) filters.severity = params.get('severity')!
  if (params.has('erlang_pid')) filters.erlang_pid = params.get('erlang_pid')!
  if (params.has('node')) filters.node = params.get('node')!
  if (params.has('subsystem')) filters.subsystem = params.get('subsystem')!
  if (params.has('labels')) filters.labels = params.get('labels')!
  if (params.has('matching_all_labels')) filters.matching_all_labels = params.get('matching_all_labels') === 'true'
  if (params.has('has_resolution_or_discussion_url')) filters.has_resolution_or_discussion_url = params.get('has_resolution_or_discussion_url') === 'true'
  if (params.has('has_doc_url')) filters.has_doc_url = params.get('has_doc_url') === 'true'

  return filters
}

function presetFiltersFromURL(): PresetQueryParams {
  const params = new URLSearchParams(window.location.search)
  const filters: PresetQueryParams = {
    limit: parseInt(params.get('preset_limit') || '1000'),
  }

  if (params.has('preset_since_time')) filters.since_time = params.get('preset_since_time')!
  if (params.has('preset_to_time')) filters.to_time = params.get('preset_to_time')!
  if (params.has('preset_node')) filters.node = params.get('preset_node')!

  return filters
}

function stateToURL(filters: QueryParams, presetFilters: PresetQueryParams, qlState: QLState, tab: Tab): string {
  const params = new URLSearchParams()

  if (tab !== 'filters') params.set('tab', tab)

  if (tab === 'filters') {
    if (filters.limit) params.set('limit', filters.limit.toString())
    if (filters.since_time) params.set('since_time', filters.since_time)
    if (filters.to_time) params.set('to_time', filters.to_time)
    if (filters.severity) params.set('severity', filters.severity)
    if (filters.erlang_pid) params.set('erlang_pid', filters.erlang_pid)
    if (filters.node) params.set('node', filters.node)
    if (filters.subsystem) params.set('subsystem', filters.subsystem)
    if (filters.labels) params.set('labels', filters.labels)
    if (filters.matching_all_labels) params.set('matching_all_labels', 'true')
    if (filters.has_resolution_or_discussion_url) params.set('has_resolution_or_discussion_url', 'true')
    if (filters.has_doc_url) params.set('has_doc_url', 'true')
  } else if (tab === 'ql') {
    if (qlState.query) params.set('ql_query', qlState.query)
    if (qlState.limit !== 1000) params.set('ql_limit', qlState.limit.toString())
  } else if (tab === 'preset_errors_or_crashes') {
    if (presetFilters.limit) params.set('preset_limit', presetFilters.limit.toString())
    if (presetFilters.since_time) params.set('preset_since_time', presetFilters.since_time)
    if (presetFilters.to_time) params.set('preset_to_time', presetFilters.to_time)
    if (presetFilters.node) params.set('preset_node', presetFilters.node)
  }

  return params.toString()
}

function App() {
  const [filters, setFilters] = useState<QueryParams>(() => filtersFromURL())
  const [presetFilters, setPresetFilters] = useState<PresetQueryParams>(() => presetFiltersFromURL())
  const [qlState, setQLState] = useState<QLState>(() => qlStateFromURL())
  const [activeTab, setActiveTab] = useState<Tab>(() => tabFromURL())
  const [qlQueryTrigger, setQLQueryTrigger] = useState(0)

  useEffect(() => {
    const handlePopState = () => {
      setFilters(filtersFromURL())
      setPresetFilters(presetFiltersFromURL())
      setQLState(qlStateFromURL())
      setActiveTab(tabFromURL())
    }

    window.addEventListener('popstate', handlePopState)
    return () => window.removeEventListener('popstate', handlePopState)
  }, [])

  useEffect(() => {
    const queryString = stateToURL(filters, presetFilters, qlState, activeTab)
    const newURL = queryString ? `?${queryString}` : window.location.pathname
    window.history.pushState({}, '', newURL)
  }, [filters, presetFilters, qlState, activeTab])

  const handlePidFilterClick = (pid: string) => {
    setFilters((prev) => ({
      ...prev,
      erlang_pid: pid,
    }))
  }

  const { data: metadata } = useQuery({
    queryKey: ['metadata'],
    queryFn: getMetadata,
  })

  const { data: stats } = useQuery({
    queryKey: ['stats'],
    queryFn: getStats,
  })

  const { data: fileMetadata } = useQuery({
    queryKey: ['fileMetadata'],
    queryFn: getFileMetadata,
  })

  const {
    data: logsData,
    isLoading,
    error,
  } = useQuery({
    queryKey: ['logs', filters],
    queryFn: () => queryLogs(filters),
    enabled: activeTab === 'filters',
  })

  const {
    data: presetLogsData,
    isLoading: isPresetLoading,
    error: presetError,
  } = useQuery({
    queryKey: ['preset_errors_or_crashes', presetFilters],
    queryFn: () => queryLogsByPreset('errors_or_crashes', presetFilters),
    enabled: activeTab === 'preset_errors_or_crashes',
  })

  const {
    data: qlLogsData,
    isLoading: isQLLoading,
    error: qlError,
  } = useQuery({
    queryKey: ['ql', qlState.query, qlState.limit, qlQueryTrigger],
    queryFn: () => queryLogsByQL({ query: qlState.query, limit: qlState.limit }),
    enabled: activeTab === 'ql' && qlState.query.trim().length > 0 && qlQueryTrigger > 0,
  })

  const handleRunQLQuery = useCallback(() => {
    setQLQueryTrigger((prev) => prev + 1)
  }, [])

  return (
    <div className="min-h-screen bg-gray-50">
      <header className="bg-white shadow-sm border-b border-gray-200">
        <div className="mx-auto px-4 sm:px-6 lg:px-8 py-4">
          <div className="flex items-center justify-between">
            <div>
              <h1 className="text-2xl font-bold text-gray-900">
                RabbitMQ Log Query Tools
              </h1>
              <p className="text-sm text-gray-500 mt-1">
                RabbitMQ logs. You dig?
              </p>
            </div>
          </div>
        </div>
      </header>

      <main className="mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="border-b border-gray-200 mb-6">
          <nav className="-mb-px flex space-x-8">
            <button
              onClick={() => setActiveTab('filters')}
              className={`${
                activeTab === 'filters'
                  ? 'border-blue-500 text-blue-600'
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
              } whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm transition-colors`}
            >
              Filters
            </button>
            <button
              onClick={() => setActiveTab('ql')}
              className={`${
                activeTab === 'ql'
                  ? 'border-blue-500 text-blue-600'
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
              } whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm transition-colors`}
            >
              QL
            </button>
            <button
              onClick={() => setActiveTab('metadata')}
              className={`${
                activeTab === 'metadata'
                  ? 'border-blue-500 text-blue-600'
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
              } whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm transition-colors`}
            >
              File Set Overview
            </button>
            <button
              onClick={() => setActiveTab('preset_errors_or_crashes')}
              className={`${
                activeTab === 'preset_errors_or_crashes'
                  ? 'border-blue-500 text-blue-600'
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
              } whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm transition-colors`}
            >
              Preset 1: Errors, Exceptions
            </button>
          </nav>
        </div>

        {fileMetadata && fileMetadata.length > 0 && stats && (
          <div className="bg-white shadow-sm border border-gray-200 rounded-lg p-4 mb-6">
            <div className="flex items-center justify-between">
              <div>
                <div className="text-sm text-gray-500">Time Range</div>
                <div className="text-sm text-gray-700 mt-1">
                  {fileMetadata[0].oldest_entry_at && fileMetadata[0].most_recent_entry_at && (
                    <>
                      {formatDate(fileMetadata[0].oldest_entry_at)} — {formatDate(fileMetadata[0].most_recent_entry_at)}
                    </>
                  )}
                </div>
              </div>
              <div className="text-right">
                <div className="text-sm text-gray-500">Total Entries</div>
                <div className="text-2xl font-bold text-gray-900 mt-1">
                  {stats.total_entries.toLocaleString()}
                </div>
              </div>
            </div>
          </div>
        )}

        {activeTab === 'filters' && (
          <div className="grid grid-cols-1 lg:grid-cols-6 gap-6">
            <div className="lg:col-span-1">
              <FilterPanel metadata={metadata || null} filters={filters} onFilterChange={setFilters} />
            </div>

            <div className="lg:col-span-5 space-y-4">
              <MetadataHeader
                fileMetadata={fileMetadata}
                onViewFullMetadata={() => setActiveTab('metadata')}
              />

              <div className="bg-white shadow-sm border border-gray-200 rounded-lg p-4">
                <div className="flex items-center justify-between">
                  <div className="text-sm text-gray-700">
                    {isLoading ? (
                      <span className="flex items-center gap-2">
                        <svg
                          className="animate-spin h-4 w-4 text-blue-600"
                          xmlns="http://www.w3.org/2000/svg"
                          fill="none"
                          viewBox="0 0 24 24"
                        >
                          <circle
                            className="opacity-25"
                            cx="12"
                            cy="12"
                            r="10"
                            stroke="currentColor"
                            strokeWidth="4"
                          />
                          <path
                            className="opacity-75"
                            fill="currentColor"
                            d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                          />
                        </svg>
                        Loading...
                      </span>
                    ) : error ? (
                      <span className="text-red-600">Error: {(error as Error).message}</span>
                    ) : (
                      <span>
                        Showing <span className="font-semibold">{logsData?.total || 0}</span> matching{' '}
                        {logsData?.total === 1 ? 'entry' : 'entries'}
                      </span>
                    )}
                  </div>
                </div>
              </div>

              <LogTable data={logsData?.entries || []} onPidFilterClick={handlePidFilterClick} />
            </div>
          </div>
        )}

        {activeTab === 'ql' && (
          <div className="space-y-4">
            <QLPanel
              query={qlState.query}
              limit={qlState.limit}
              onQueryChange={(query) => setQLState((prev) => ({ ...prev, query }))}
              onLimitChange={(limit) => setQLState((prev) => ({ ...prev, limit }))}
              onRunQuery={handleRunQLQuery}
              isLoading={isQLLoading}
            />

            <div className="bg-white shadow-sm border border-gray-200 rounded-lg p-4">
              <div className="flex items-center justify-between">
                <div className="text-sm text-gray-700">
                  {isQLLoading ? (
                    <span className="flex items-center gap-2">
                      <svg
                        className="animate-spin h-4 w-4 text-blue-600"
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                      >
                        <circle
                          className="opacity-25"
                          cx="12"
                          cy="12"
                          r="10"
                          stroke="currentColor"
                          strokeWidth="4"
                        />
                        <path
                          className="opacity-75"
                          fill="currentColor"
                          d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                        />
                      </svg>
                      Running query...
                    </span>
                  ) : qlError ? (
                    <span className="text-red-600">Error: {(qlError as Error).message}</span>
                  ) : qlLogsData ? (
                    <span>
                      Showing <span className="font-semibold">{qlLogsData.total}</span> matching{' '}
                      {qlLogsData.total === 1 ? 'entry' : 'entries'}
                    </span>
                  ) : (
                    <span className="text-gray-500">Enter a query and click Run</span>
                  )}
                </div>
              </div>
            </div>

            <LogTable data={qlLogsData?.entries || []} onPidFilterClick={handlePidFilterClick} />
          </div>
        )}

        {activeTab === 'preset_errors_or_crashes' && (
          <div className="grid grid-cols-1 lg:grid-cols-6 gap-6">
            <div className="lg:col-span-1">
              <PresetFilterPanel
                metadata={metadata || null}
                filters={presetFilters}
                onFilterChange={setPresetFilters}
              />
            </div>

            <div className="lg:col-span-5 space-y-4">
              <div className="bg-white shadow-sm border border-gray-200 rounded-lg p-4">
                <div className="flex items-center justify-between">
                  <div className="text-sm text-gray-700">
                    {isPresetLoading ? (
                      <span className="flex items-center gap-2">
                        <svg
                          className="animate-spin h-4 w-4 text-blue-600"
                          xmlns="http://www.w3.org/2000/svg"
                          fill="none"
                          viewBox="0 0 24 24"
                        >
                          <circle
                            className="opacity-25"
                            cx="12"
                            cy="12"
                            r="10"
                            stroke="currentColor"
                            strokeWidth="4"
                          />
                          <path
                            className="opacity-75"
                            fill="currentColor"
                            d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                          />
                        </svg>
                        Loading...
                      </span>
                    ) : presetError ? (
                      <span className="text-red-600">Error: {(presetError as Error).message}</span>
                    ) : (
                      <span>
                        Showing <span className="font-semibold">{presetLogsData?.total || 0}</span> matching{' '}
                        {presetLogsData?.total === 1 ? 'entry' : 'entries'}
                      </span>
                    )}
                  </div>
                </div>
              </div>

              <LogTable data={presetLogsData?.entries || []} onPidFilterClick={handlePidFilterClick} />
            </div>
          </div>
        )}

        {activeTab === 'metadata' && <FileMetadataTab fileMetadata={fileMetadata} />}
      </main>
    </div>
  )
}

export default App
