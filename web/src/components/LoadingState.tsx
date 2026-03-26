import { Skeleton } from '@/components/ui/skeleton';

export function TaskTableSkeleton() {
  return (
    <div className="space-y-3">
      {Array.from({ length: 6 }, (_, index) => <Skeleton className="h-14 w-full" key={index} />)}
    </div>
  );
}

export function PageSkeleton() {
  return (
    <div className="space-y-5 p-8">
      <Skeleton className="h-20 w-2/3" />
      <div className="grid grid-cols-5 gap-3">
        {Array.from({ length: 5 }, (_, index) => <Skeleton className="h-24" key={index} />)}
      </div>
      <Skeleton className="h-[420px]" />
    </div>
  );
}
