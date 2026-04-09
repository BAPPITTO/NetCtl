import React, { useState, useEffect, useCallback } from 'react';
import { Vlan, createVlan, deleteVlan } from '../api';

interface VLANManagerProps {
  vlans: Record<number, Vlan>;
}

export default function VLANManager({ vlans }: VLANManagerProps) {
  const [vlanList, setVlanList] = useState<Vlan[]>([]);
  const [showForm, setShowForm] = useState(false);
  const [formData, setFormData] = useState({
    vlan_id: '',
    name: '',
    subnet: '',
    gateway: '',
  });
  const [errors, setErrors] = useState<{ vlan_id?: string; name?: string; subnet?: string; gateway?: string }>({});

  // Validate inputs
  useEffect(() => {
    const newErrors: typeof errors = {};
    if (!formData.vlan_id || Number(formData.vlan_id) <= 0) newErrors.vlan_id = 'VLAN ID must be > 0';
    if (!formData.name) newErrors.name = 'Name is required';
    const subnetRegex = /^(\d{1,3}\.){3}\d{1,3}\/\d{1,2}$/;
    if (!subnetRegex.test(formData.subnet)) newErrors.subnet = 'Invalid subnet format';
    const gatewayRegex = /^(\d{1,3}\.){3}\d{1,3}$/;
    if (!gatewayRegex.test(formData.gateway)) newErrors.gateway = 'Invalid gateway format';
    setErrors(newErrors);
  }, [formData]);

  // Auto-refresh VLAN list every 3 seconds
  useEffect(() => {
    const refresh = () => setVlanList(Object.values(vlans));
    refresh();
    const interval = setInterval(refresh, 3000);
    return () => clearInterval(interval);
  }, [vlans]);

  const handleCreate = useCallback(async (e: React.FormEvent) => {
    e.preventDefault();
    if (Object.keys(errors).length > 0) return;

    try {
      await createVlan({
        vlan_id: Number(formData.vlan_id),
        name: formData.name,
        subnet: formData.subnet,
        gateway: formData.gateway,
        dhcp_enabled: true,
      });
      setFormData({ vlan_id: '', name: '', subnet: '', gateway: '' });
      setShowForm(false);
    } catch (err) {
      console.error('Failed to create VLAN:', err);
    }
  }, [formData, errors]);

  const handleDelete = useCallback(async (vlanId: number) => {
    if (!confirm(`Delete VLAN ${vlanId}?`)) return;
    try {
      await deleteVlan(vlanId);
    } catch (err) {
      console.error('Failed to delete VLAN:', err);
    }
  }, []);

  const inputStyle = (field: keyof typeof formData) => ({
    border: `1px solid ${errors[field] ? '#ff0000' : '#00ff00'}`,
    background: '#001100',
    color: '#00ff00',
    padding: '6px 10px',
    borderRadius: '4px',
    fontFamily: 'monospace',
    width: '100%',
    marginBottom: '6px',
  });

  const isSubmitDisabled = Object.keys(errors).length > 0;

  return (
    <section style={{ padding: '20px', background: '#010101', borderRadius: '6px', border: '1px solid #00ff0033', color: '#00ff00', fontFamily: 'monospace' }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '15px' }}>
        <h2>VLAN Management ({vlanList.length})</h2>
        <button onClick={() => setShowForm(!showForm)} style={{ background: '#00ff0033', border: '1px solid #00ff0044', padding: '6px 12px', borderRadius: '4px', cursor: 'pointer' }}>
          {showForm ? '− Close' : '+ Create VLAN'}
        </button>
      </div>

      {showForm && (
        <form onSubmit={handleCreate} style={{ marginBottom: '20px' }}>
          <input type="number" placeholder="VLAN ID" value={formData.vlan_id} onChange={(e) => setFormData({ ...formData, vlan_id: e.target.value })} style={inputStyle('vlan_id')} />
          {errors.vlan_id && <div style={{ color: '#ff4444', fontSize: '0.8em' }}>{errors.vlan_id}</div>}

          <input type="text" placeholder="Name" value={formData.name} onChange={(e) => setFormData({ ...formData, name: e.target.value })} style={inputStyle('name')} />
          {errors.name && <div style={{ color: '#ff4444', fontSize: '0.8em' }}>{errors.name}</div>}

          <input type="text" placeholder="Subnet (e.g., 192.168.10.0/24)" value={formData.subnet} onChange={(e) => setFormData({ ...formData, subnet: e.target.value })} style={inputStyle('subnet')} />
          {errors.subnet && <div style={{ color: '#ff4444', fontSize: '0.8em' }}>{errors.subnet}</div>}

          <input type="text" placeholder="Gateway (e.g., 192.168.10.1)" value={formData.gateway} onChange={(e) => setFormData({ ...formData, gateway: e.target.value })} style={inputStyle('gateway')} />
          {errors.gateway && <div style={{ color: '#ff4444', fontSize: '0.8em' }}>{errors.gateway}</div>}

          <button type="submit" disabled={isSubmitDisabled} style={{
            background: isSubmitDisabled ? '#00220044' : '#00ff0022',
            border: '1px solid #00ff0044',
            padding: '8px 16px',
            borderRadius: '4px',
            cursor: isSubmitDisabled ? 'not-allowed' : 'pointer',
          }}>Create VLAN</button>
          <button type="button" onClick={() => setShowForm(false)} style={{ marginLeft: '6px', background: '#00220044', border: '1px solid #00220055', padding: '8px 16px', borderRadius: '4px', cursor: 'pointer' }}>Cancel</button>
        </form>
      )}

      <div>
        {vlanList.map((vlan) => (
          <div key={vlan.id} style={{ background: '#001100', border: '1px solid #00ff0033', borderRadius: '6px', padding: '12px', marginBottom: '12px' }}>
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
              <h3>VLAN {vlan.id}: {vlan.name}</h3>
              <button onClick={() => handleDelete(vlan.id)} style={{ background: '#ff000055', border: '1px solid #ff000077', color: '#ff0000', padding: '4px 8px', borderRadius: '4px', cursor: 'pointer' }}>Delete</button>
            </div>
            <div style={{ marginTop: '8px' }}>
              <p><strong>Interface:</strong> {vlan.interface}</p>
              <p><strong>Subnet:</strong> {vlan.subnet}</p>
              <p><strong>Gateway:</strong> {vlan.gateway}</p>
              <p><strong>DHCP:</strong> {vlan.dhcp_enabled ? 'Enabled' : 'Disabled'}</p>
            </div>
          </div>
        ))}
        {vlanList.length === 0 && <p style={{ opacity: 0.6, textAlign: 'center', marginTop: '20px' }}>No VLANs configured</p>}
      </div>
    </section>
  );
}