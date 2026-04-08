import { useEffect, useState } from 'react';

interface MetricsUpdate {
  mac: string;
  packets_sent: number;
  packets_dropped: number;
  bytes_sent: number;
  bytes_received: number;
  current_rate_mbps: number;
}

export function useMetricsStream() {
  const [metrics, setMetrics] = useState<MetricsUpdate[]>([]);
  const [isConnected, setIsConnected] = useState(false);

  useEffect(() => {
    const eventSource = new EventSource('/api/metrics/stream');

    eventSource.onopen = () => {
      setIsConnected(true);
    };

    eventSource.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        setMetrics((prev) => {
          const map = new Map(prev.map((m) => [m.mac, m]));
          map.set(data.mac, data);
          return Array.from(map.values());
        });
      } catch (error) {
        console.error('Failed to parse metrics:', error);
      }
    };

    eventSource.onerror = () => {
      setIsConnected(false);
      eventSource.close();
    };

    return () => {
      eventSource.close();
    };
  }, []);

  return { metrics, isConnected };
}
