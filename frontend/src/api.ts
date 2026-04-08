import axios from 'axios';

const API_BASE = '/api';

export const api = axios.create({
  baseURL: API_BASE,
  timeout: 10000,
});

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

export async function getState() {
  const { data } = await api.get('/state');
  return data;
}

export async function getInterfaces() {
  const { data } = await api.get('/interfaces');
  return data;
}

export async function createVlan(vlan: any) {
  const { data } = await api.post('/vlan', vlan);
  return data;
}

export async function deleteVlan(vlanId: number) {
  const { data } = await api.delete(`/vlan/${vlanId}`);
  return data;
}

export async function getDevices() {
  const { data } = await api.get('/devices');
  return data;
}

export async function createDevice(device: any) {
  const { data } = await api.post('/devices', device);
  return data;
}

export async function setQosRule(mac: string, rateMbps: number) {
  const { data } = await api.post(`/qos/device/${mac}`, { mac, rate_mbps: rateMbps });
  return data;
}

export async function removeQosRule(mac: string) {
  const { data } = await api.delete(`/qos/device/${mac}`);
  return data;
}

export async function getQosRules() {
  const { data } = await api.get('/qos/devices');
  return data;
}

export async function getMetricsSummary() {
  const { data } = await api.get('/metrics/summary');
  return data;
}

export async function healthCheck() {
  const { data } = await api.get('/health');
  return data;
}
