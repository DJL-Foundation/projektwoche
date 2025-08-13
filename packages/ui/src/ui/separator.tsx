"use client";

import * as React from "react";
import * as SeparatorPrimitive from "@radix-ui/react-separator";

import { cn } from "../lib/utils";

const Separator = React.forwardRef<
  React.ElementRef<typeof SeparatorPrimitive.Root>,
  React.ComponentPropsWithoutRef<typeof SeparatorPrimitive.Root>
>(
  (
    { className, orientation = "horizontal", decorative = true, ...props },
    ref,
  ) => (
    <SeparatorPrimitive.Root
      ref={ref}
      decorative={decorative}
      orientation={orientation}
      className={cn(
        "shrink-0 bg-border",
        orientation === "horizontal" ? "h-[1px] w-full" : "h-full w-[1px]",
        className,
      )}
      {...props}
    />
  ),
);
Separator.displayName = SeparatorPrimitive.Root.displayName;

export { Separator };

interface LabeledSeparatorProps extends React.HTMLAttributes<HTMLDivElement> {
  label: string;
}

const LabeledSeparator: React.FC<LabeledSeparatorProps> = ({
  label,
  className,
  ...props
}) => (
  <div className={cn("relative flex items-center", className)} {...props}>
    <Separator className="flex-1" />
    <span className="mx-2 bg-background px-2 text-xs text-muted-foreground uppercase">
      {label}
    </span>
    <Separator className="flex-1" />
  </div>
);

export { LabeledSeparator };
