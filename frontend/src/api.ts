import axios, { AxiosInstance } from 'axios';

const API_BASE = '/api';

export const api: AxiosInstance = axios.create({
  baseURL: API_BASE,
  timeout: 10000,
});

// ---- Types ----
export interface Device {
  id: string;
  mac: string;
  name: string;
  vlan_id?: number;
  rate_limit_mbps?: number;
  blocked: boolean;
  created_at: string;
  last_seen?: string;
}

export interface Vlan {
  id: number;
  name: string;
  subnet: string;
  gateway: string;
  dhcp_enabled: boolean;
  interface: string;
  created_at: string;
}

// ---- API Wrappers ----
async function handleRequest<T>(promise: Promise<{ data: T }>): Promise<T> {
  try {
    const { data } = await promise;
    return data;
  } catch (err) {
    console.error('API error:', err);
    throw err;
  }
}

export async function getState() {
  return handleRequest(api.get('/state'));
}

export async function getInterfaces() {
  return handleRequest(api.get('/interfaces'));
}

export async function createVlan(vlan: Partial<Vlan>) {
  return handleRequest(api.post('/vlan', vlan));
}

export async function deleteVlan(vlanId: number) {
  return handleRequest(api.delete(`/vlan/${vlanId}`));
}

export async function getDevices() {
  return handleRequest(api.get('/devices'));
}

export async function createDevice(device: Partial<Device>) {
  return handleRequest(api.post('/devices', device));
}

export async function setQosRule(mac: string, rateMbps: number) {
  return handleRequest(api.post(`/qos/device/${mac}`, { mac, rate_mbps: rateMbps }));
}

export async function removeQosRule(mac: string) {
  return handleRequest(api.delete(`/qos/device/${mac}`));
}

export async function getQosRules() {
  return handleRequest(api.get<Record<string, number>>('/qos/devices'));
}

export async function getMetricsSummary() {
  return handleRequest(api.get('/metrics/summary'));
}

export async function healthCheck() {
  return handleRequest(api.get('/health'));
}