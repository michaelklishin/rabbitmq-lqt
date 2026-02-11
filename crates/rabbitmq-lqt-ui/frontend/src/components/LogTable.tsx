import {
  useReactTable,
  getCoreRowModel,
  flexRender,
  ColumnDef,
  VisibilityState,
} from '@tanstack/react-table'
import { LogEntry } from '../api/client'
import { format } from 'date-fns'
import { useState } from 'react'

interface LogTableProps {
  data: LogEntry[]
  onPidFilterClick?: (pid: string) => void
}

const severityColors: Record<string, string> = {
  debug: 'text-gray-500',
  info: 'text-blue-600',
  notice: 'text-cyan-600',
  warning: 'text-yellow-600',
  error: 'text-red-600',
  critical: 'text-red-800 font-bold',
}

const createColumns = (onPidFilterClick?: (pid: string) => void): ColumnDef<LogEntry>[] => [
  {
    id: 'id',
    accessorKey: 'id',
    header: 'ID',
    cell: (info) => (
      <div className="text-sm text-gray-600 font-mono">{info.getValue() as number}</div>
    ),
    size: 45,
  },
  {
    id: 'timestamp',
    accessorKey: 'timestamp',
    header: 'Timestamp',
    cell: (info) => {
      const timestamp = info.getValue() as string
      const date = new Date(timestamp)
      return (
        <div className="text-sm text-gray-900 whitespace-nowrap font-mono">
          {format(date, 'MMM d, yyyy HH:mm:ss.SSS')}
        </div>
      )
    },
    size: 200,
  },
  {
    id: 'severity',
    accessorKey: 'severity',
    header: 'Severity',
    cell: (info) => {
      const severity = info.getValue() as string
      const colorClass = severityColors[severity] || 'text-gray-900'
      return (
        <div className={`text-sm font-medium ${colorClass}`}>
          {severity}
        </div>
      )
    },
    size: 75,
  },
  {
    id: 'node',
    accessorKey: 'node',
    header: 'Node',
    cell: (info) => (
      <div className="text-sm text-gray-700 font-mono truncate">
        {info.getValue() as string}
      </div>
    ),
    size: 120,
  },
  {
    id: 'erlang_pid',
    accessorKey: 'erlang_pid',
    header: 'PID',
    cell: (info) => {
      const pid = info.getValue() as string
      return (
        <div className="flex items-center gap-2">
          <span className="text-sm text-gray-600 font-mono">{pid}</span>
          {onPidFilterClick && (
            <button
              onClick={() => onPidFilterClick(pid)}
              className="text-xs text-blue-600 hover:text-blue-800 hover:underline"
            >
              Filter
            </button>
          )}
        </div>
      )
    },
    size: 70,
  },
  {
    id: 'subsystem',
    accessorKey: 'subsystem',
    header: 'Subsystem',
    cell: (info) => {
      const subsystem = info.getValue() as string | null
      return (
        <div className="text-sm text-gray-700">
          {subsystem || '-'}
        </div>
      )
    },
    size: 110,
  },
  {
    id: 'message',
    accessorKey: 'message',
    header: 'Message',
    cell: (info) => {
      const message = info.getValue() as string
      return (
        <div className="text-sm text-gray-900 whitespace-pre font-mono max-w-3xl overflow-x-auto">
          {message}
        </div>
      )
    },
    size: 640,
  },
  {
    id: 'labels',
    accessorKey: 'labels',
    header: 'Labels',
    cell: (info) => {
      const labels = info.getValue() as Record<string, boolean>
      const activeLabels = Object.entries(labels)
        .filter(([_, value]) => value)
        .map(([key]) => key)
        .sort()

      return (
        <div className="flex flex-wrap gap-1">
          {activeLabels.map((label) => (
            <span
              key={label}
              className="inline-block px-2 py-1 text-xs bg-blue-100 text-blue-800 rounded"
            >
              {label}
            </span>
          ))}
        </div>
      )
    },
    size: 180,
  },
  {
    id: 'doc_url',
    accessorKey: 'doc_url',
    header: 'Doc URL',
    cell: (info) => {
      const url = info.getValue() as string | null
      return url ? (
        <a
          href={url}
          target="_blank"
          rel="noopener noreferrer"
          className="text-sm text-blue-600 hover:text-blue-800 underline"
        >
          View
        </a>
      ) : (
        <span className="text-sm text-gray-400">-</span>
      )
    },
    size: 75,
  },
  {
    id: 'resolution_or_discussion_url',
    accessorKey: 'resolution_or_discussion_url',
    header: 'Resolution',
    cell: (info) => {
      const url = info.getValue() as string | null
      return url ? (
        <a
          href={url}
          target="_blank"
          rel="noopener noreferrer"
          className="text-sm text-blue-600 hover:text-blue-800 underline"
        >
          View
        </a>
      ) : (
        <span className="text-sm text-gray-400">-</span>
      )
    },
    size: 85,
  },
]

const defaultColumnVisibility: VisibilityState = {
  doc_url: false,
}

export function LogTable({ data, onPidFilterClick }: LogTableProps) {
  const [columnVisibility, setColumnVisibility] = useState<VisibilityState>(defaultColumnVisibility)
  const [showColumnControls, setShowColumnControls] = useState(false)

  const columns = createColumns(onPidFilterClick)

  const table = useReactTable({
    data,
    columns,
    state: {
      columnVisibility,
    },
    onColumnVisibilityChange: setColumnVisibility,
    getCoreRowModel: getCoreRowModel(),
  })

  const resetColumns = () => {
    setColumnVisibility(defaultColumnVisibility)
  }

  return (
    <div className="bg-white shadow-sm border border-gray-200 rounded-lg overflow-hidden">
      <div className="p-4 border-b border-gray-200 flex items-center justify-between bg-gray-50">
        <div className="flex items-center gap-3">
          <button
            onClick={() => setShowColumnControls(!showColumnControls)}
            className="flex items-center gap-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded font-medium text-sm transition-colors"
          >
            <svg
              className="w-4 h-4"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M9 17V7m0 10a2 2 0 01-2 2H5a2 2 0 01-2-2V7a2 2 0 012-2h2a2 2 0 012 2m0 10a2 2 0 002 2h2a2 2 0 002-2M9 7a2 2 0 012-2h2a2 2 0 012 2m0 0h6m-6 0v10"
              />
            </svg>
            {showColumnControls ? 'Done' : 'Select Columns'}
          </button>
          {!showColumnControls && (
            <span className="text-xs text-gray-500">
              {table.getVisibleLeafColumns().length} of {table.getAllLeafColumns().length} columns visible
            </span>
          )}
        </div>
        {showColumnControls && (
          <div className="flex items-center gap-2">
            <button
              onClick={() => table.toggleAllColumnsVisible(true)}
              className="text-sm px-3 py-1.5 bg-white border border-gray-300 hover:bg-gray-50 rounded font-medium"
            >
              Show All
            </button>
            <button
              onClick={resetColumns}
              className="text-sm px-3 py-1.5 bg-white border border-gray-300 hover:bg-gray-50 rounded font-medium"
            >
              Reset Columns
            </button>
          </div>
        )}
      </div>

      {showColumnControls && (
        <div className="p-4 border-b border-gray-200 bg-gray-50">
          <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-5 gap-3">
            {table.getAllLeafColumns().map((column) => (
              <label key={column.id} className="flex items-center space-x-2 text-sm">
                <input
                  type="checkbox"
                  checked={column.getIsVisible()}
                  onChange={column.getToggleVisibilityHandler()}
                  className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                />
                <span className="text-gray-700">{column.id}</span>
              </label>
            ))}
          </div>
        </div>
      )}

      <div className="overflow-x-auto">
        <table className="min-w-full divide-y divide-gray-200">
          <thead className="bg-gray-50">
            {table.getHeaderGroups().map((headerGroup) => (
              <tr key={headerGroup.id}>
                {headerGroup.headers.map((header) => (
                  <th
                    key={header.id}
                    className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                    style={{ width: header.getSize() }}
                  >
                    {header.isPlaceholder
                      ? null
                      : flexRender(header.column.columnDef.header, header.getContext())}
                  </th>
                ))}
              </tr>
            ))}
          </thead>
          <tbody className="bg-white divide-y divide-gray-200">
            {table.getRowModel().rows.map((row) => (
              <tr key={row.id} className="hover:bg-gray-50">
                {row.getVisibleCells().map((cell) => (
                  <td key={cell.id} className="px-4 py-3">
                    {flexRender(cell.column.columnDef.cell, cell.getContext())}
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {data.length === 0 && (
        <div className="text-center py-12 text-gray-500">
          No log entries found. Try adjusting your filters.
        </div>
      )}
    </div>
  )
}
