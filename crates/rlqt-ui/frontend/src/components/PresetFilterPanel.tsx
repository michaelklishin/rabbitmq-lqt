import { MetadataResponse, PresetQueryParams } from '../api/client'

interface PresetFilterPanelProps {
  metadata: MetadataResponse | null
  filters: PresetQueryParams
  onFilterChange: (filters: PresetQueryParams) => void
}

export function PresetFilterPanel({ metadata, filters, onFilterChange }: PresetFilterPanelProps) {
  const updateFilter = (key: keyof PresetQueryParams, value: any) => {
    onFilterChange({
      ...filters,
      [key]: value,
    })
  }

  const clearFilters = () => {
    onFilterChange({ limit: 1000 })
  }

  return (
    <div className="bg-white shadow-sm border border-gray-200 rounded-lg p-6 space-y-6">
      <div className="flex items-center justify-between">
        <h2 className="text-lg font-semibold text-gray-900">Filters</h2>
        <button
          onClick={clearFilters}
          className="text-sm text-blue-600 hover:text-blue-800"
        >
          Revert to Default View
        </button>
      </div>

      <div className="space-y-6">
        <div className="bg-blue-50 border border-blue-200 rounded-md p-3">
          <h3 className="text-sm font-semibold text-blue-900 mb-1">Preset: Errors, Exceptions</h3>
          <p className="text-xs text-blue-700">
            Shows log entries with error severity OR entries labelled as erl_process_crash or exceptions
          </p>
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
          <h3 className="text-sm font-semibold text-gray-900 mb-3">Node Filter</h3>
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
