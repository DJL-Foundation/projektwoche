import type React from "react";
import Link from "next/link";
import Image from "next/image";
import { ThemeToggle } from "../theme-toggle";
import { motion, MotionConfig } from "motion/react";
import { Button } from "../ui/button";

interface HeaderProps extends React.HTMLAttributes<HTMLDivElement> {
  beta?: boolean; // Shows beta string
  print?: boolean; // Print Styles and Full Text
}

export default function Header({
  beta = false,
  print = false,
  ...props
}: HeaderProps) {
  return (
    <MotionConfig reducedMotion={print ? "always" : "user"}>
      <motion.header
        className={`bg-background ${print ? "border-b-2" : "border-b border-border"}`}
        initial={{ opacity: 0, y: -20 }}
        animate={{ opacity: 1, y: 0 }}
      >
        <div className="container mx-auto px-4 py-4" {...props}>
          {!print ? (
            <div className="grid grid-cols-3 items-center">
              {/* Left section - Logo and title */}
              <div className="flex items-center space-x-4">
                <Link href="/" className="flex items-center space-x-2" prefetch>
                  <div className="relative w-10 h-10">
                    <Image
                      src={"/logo.png"}
                      alt="Hackclub Stade Logo"
                      fill
                      className="object-contain"
                    />
                  </div>
                  <span className="text-xl font-semibold">
                    Hackclub Stade - Projektwoche
                  </span>
                  {beta && (
                    <span className="text-xl font-semibold">&lt;Beta&gt;</span>
                  )}
                </Link>
              </div>

              {/* Center section - Projekte button */}
              <div className="flex justify-center">
                <Button variant="outline" asChild>
                  <Link href="/projekte" className="flex items-center gap-2">
                    Projekte
                  </Link>
                </Button>
              </div>

              {/* Right section - Theme toggle */}
              <div className="flex justify-end">
                <ThemeToggle />
              </div>
            </div>
          ) : (
            // Print layout remains the same
            <div className="flex items-center justify-between">
              <div className="flex items-center space-x-4">
                <Link href="/" className="flex items-center space-x-2" prefetch>
                  <div className="relative w-10 h-10">
                    <Image
                      src={"/logo.png"}
                      alt="Hackclub Stade Logo"
                      fill
                      className="object-contain brightness-0"
                    />
                  </div>
                  <span className="text-xl font-semibold">
                    Hackclub Stade - Projektwoche
                  </span>
                  {beta && (
                    <span className="text-xl font-semibold">&lt;Beta&gt;</span>
                  )}
                  <span className="text-sm font-medium">
                    by The DJL Foundation
                  </span>
                </Link>
              </div>
            </div>
          )}
        </div>
      </motion.header>
    </MotionConfig>
  );
}
