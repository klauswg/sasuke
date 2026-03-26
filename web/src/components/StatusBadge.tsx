import type React from 'react';
import { Badge } from '@/components/ui/badge';
import { statusBadgeClass } from '@/lib/status';

interface StatusBadgeProps {
  value?: string | null;
  tone?: string;
  label?: React.ReactNode;
}

export function StatusBadge({ value, tone, label }: StatusBadgeProps) {
  if (!value) return null;
  return <Badge variant="outline" className={statusBadgeClass(value, tone)}>{label ?? value}</Badge>;
}
