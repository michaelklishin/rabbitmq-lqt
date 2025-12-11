export interface LogEntry {
  id: number
  node: string
  timestamp: string
  severity: string
  erlang_pid: string
  message: string
  subsystem: string | null
  labels: Record<string, boolean>
  doc_url: string | null
  resolution_or_discussion_url: string | null
}

export interface LogQueryResponse {
  entries: LogEntry[]
  total: number
}

export interface MetadataResponse {
  severities: string[]
  subsystems: string[]
  labels: string[]
  nodes: string[]
}

export interface StatsResponse {
  total_entries: number
  nodes: Array<{
    node: string
    count: number
  }>
}

export interface FileMetadataResponse {
  file_path: string
  rabbitmq_versions: string[]
  erlang_versions: string[]
  tls_library: string | null
  oldest_entry_at: string | null
  most_recent_entry_at: string | null
  total_lines: number
  total_entries: number
  nodes: string[]
  subsystems: string[]
  labels: string[]
  enabled_plugins: string[]
}

export interface QueryParams {
  since_time?: string
  to_time?: string
  severity?: string
  erlang_pid?: string
  node?: string
  subsystem?: string
  labels?: string
  matching_all_labels?: boolean
  limit?: number
  has_resolution_or_discussion_url?: boolean
  has_doc_url?: boolean
}

const API_BASE = '/api'

export async function queryLogs(params: QueryParams): Promise<LogQueryResponse> {
  const queryString = new URLSearchParams(
    Object.entries(params)
      .filter(([_, v]) => v !== undefined && v !== null && v !== '')
      .map(([k, v]) => [k, String(v)])
  ).toString()

  const url = `${API_BASE}/logs${queryString ? `?${queryString}` : ''}`
  const response = await fetch(url)

  if (!response.ok) {
    throw new Error(`Failed to query logs: ${response.statusText}`)
  }

  return response.json()
}

export async function getMetadata(): Promise<MetadataResponse> {
  const response = await fetch(`${API_BASE}/metadata`)

  if (!response.ok) {
    throw new Error(`Failed to fetch metadata: ${response.statusText}`)
  }

  return response.json()
}

export async function getStats(): Promise<StatsResponse> {
  const response = await fetch(`${API_BASE}/stats`)

  if (!response.ok) {
    throw new Error(`Failed to fetch stats: ${response.statusText}`)
  }

  return response.json()
}

export async function getFileMetadata(): Promise<FileMetadataResponse[]> {
  const response = await fetch(`${API_BASE}/file-metadata`)

  if (!response.ok) {
    throw new Error(`Failed to fetch file metadata: ${response.statusText}`)
  }

  return response.json()
}

export interface PresetQueryParams {
  since_time?: string
  to_time?: string
  node?: string
  limit?: number
}

export async function queryLogsByPreset(
  presetName: string,
  params: PresetQueryParams = {}
): Promise<LogQueryResponse> {
  const queryString = new URLSearchParams(
    Object.entries(params)
      .filter(([_, v]) => v !== undefined && v !== null && v !== '')
      .map(([k, v]) => [k, String(v)])
  ).toString()

  const url = `${API_BASE}/logs/preset/${presetName}${queryString ? `?${queryString}` : ''}`
  const response = await fetch(url)

  if (!response.ok) {
    throw new Error(`Failed to query logs by preset: ${response.statusText}`)
  }

  return response.json()
}
