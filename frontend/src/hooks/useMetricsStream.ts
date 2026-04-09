import { useEffect, useState, useRef } from 'react';

export interface MetricsUpdate {
  mac: string;
  packets_sent: number;
  packets_dropped: number;
  bytes_sent: number;
  bytes_received: number;
  current_rate_mbps: number;
}

interface UseMetricsStreamResult {
  metrics: MetricsUpdate[];
  isConnected: boolean;
}

export function useMetricsStream(): UseMetricsStreamResult {
  const [metrics, setMetrics] = useState<MetricsUpdate[]>([]);
  const [isConnected, setIsConnected] = useState(false);
  const eventSourceRef = useRef<EventSource | null>(null);
  const reconnectTimer = useRef<number | null>(null);

  useEffect(() => {
    const connect = () => {
      const eventSource = new EventSource('/api/metrics/stream');
      eventSourceRef.current = eventSource;

      eventSource.onopen = () => {
        setIsConnected(true);
        if (reconnectTimer.current) {
          clearTimeout(reconnectTimer.current);
          reconnectTimer.current = null;
        }
      };

      eventSource.onmessage = (event) => {
        try {
          const data: MetricsUpdate = JSON.parse(event.data);

          setMetrics((prev) => {
            const map = new Map(prev.map((m) => [m.mac, m]));
            map.set(data.mac, data);
            return Array.from(map.values());
          });
        } catch (err) {
          console.error('Failed to parse metrics:', err);
        }
      };

      eventSource.onerror = () => {
        setIsConnected(false);
        eventSource.close();

        // Attempt to reconnect after 2 seconds
        if (!reconnectTimer.current) {
          reconnectTimer.current = window.setTimeout(() => {
            connect();
          }, 2000);
        }
      };
    };

    connect();

    return () => {
      if (eventSourceRef.current) eventSourceRef.current.close();
      if (reconnectTimer.current) clearTimeout(reconnectTimer.current);
    };
  }, []);

  return { metrics, isConnected };
}