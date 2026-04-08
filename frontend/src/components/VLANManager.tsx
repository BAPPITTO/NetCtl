import React, { useState, useEffect } from 'react';
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

  useEffect(() => {
    setVlanList(Object.values(vlans));
  }, [vlans]);

  const handleCreate = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await createVlan({
        vlan_id: parseInt(formData.vlan_id),
        name: formData.name,
        subnet: formData.subnet,
        gateway: formData.gateway,
        dhcp_enabled: true,
      });
      setFormData({ vlan_id: '', name: '', subnet: '', gateway: '' });
      setShowForm(false);
    } catch (error) {
      console.error('Failed to create VLAN:', error);
    }
  };

  const handleDelete = async (vlanId: number) => {
    if (confirm(`Delete VLAN ${vlanId}?`)) {
      try {
        await deleteVlan(vlanId);
      } catch (error) {
        console.error('Failed to delete VLAN:', error);
      }
    }
  };

  return (
    <section className="content-panel">
      <div className="panel-header">
        <h2>VLAN Management</h2>
        <button className="btn-primary" onClick={() => setShowForm(!showForm)}>
          + Create VLAN
        </button>
      </div>

      {showForm && (
        <form className="vlan-form" onSubmit={handleCreate}>
          <input
            type="number"
            placeholder="VLAN ID"
            value={formData.vlan_id}
            onChange={(e) => setFormData({ ...formData, vlan_id: e.target.value })}
            required
          />
          <input
            type="text"
            placeholder="Name"
            value={formData.name}
            onChange={(e) => setFormData({ ...formData, name: e.target.value })}
            required
          />
          <input
            type="text"
            placeholder="Subnet (e.g., 192.168.10.0/24)"
            value={formData.subnet}
            onChange={(e) => setFormData({ ...formData, subnet: e.target.value })}
            required
          />
          <input
            type="text"
            placeholder="Gateway (e.g., 192.168.10.1)"
            value={formData.gateway}
            onChange={(e) => setFormData({ ...formData, gateway: e.target.value })}
            required
          />
          <button type="submit">Create</button>
          <button type="button" onClick={() => setShowForm(false)}>Cancel</button>
        </form>
      )}

      <div className="vlan-list">
        {vlanList.map((vlan) => (
          <div key={vlan.id} className="vlan-card">
            <div className="vlan-header">
              <h3>VLAN {vlan.id}: {vlan.name}</h3>
              <button className="btn-delete" onClick={() => handleDelete(vlan.id)}>Delete</button>
            </div>
            <div className="vlan-details">
              <p><strong>Interface:</strong> {vlan.interface}</p>
              <p><strong>Subnet:</strong> {vlan.subnet}</p>
              <p><strong>Gateway:</strong> {vlan.gateway}</p>
              <p><strong>DHCP:</strong> {vlan.dhcp_enabled ? 'Enabled' : 'Disabled'}</p>
            </div>
          </div>
        ))}
      </div>
    </section>
  );
}
