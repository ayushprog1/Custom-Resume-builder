interface CardProps {
  className?: string;
  children: React.ReactNode;
}

export function Card({ className = "", children }: CardProps) {
  return (
    <div className={`rounded-lg border border-border bg-card text-card-foreground shadow-sm ${className}`}>
      {children}
    </div>
  );
}

export function CardHeader({ className = "", children }: CardProps) {
  return <div className={`flex flex-col space-y-1.5 p-4 ${className}`}>{children}</div>;
}

export function CardTitle({ className = "", children }: CardProps) {
  return <h3 className={`text-lg font-semibold leading-none tracking-tight ${className}`}>{children}</h3>;
}

export function CardContent({ className = "", children }: CardProps) {
  return <div className={`p-4 pt-0 ${className}`}>{children}</div>;
}
