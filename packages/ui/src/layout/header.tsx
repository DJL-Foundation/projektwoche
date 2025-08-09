import Link from "next/link";
import Image from "next/image";
import { ThemeToggle } from "../theme-toggle";
import { motion, MotionConfig } from "motion/react";

interface HeaderProps extends React.HTMLAttributes<HTMLDivElement> {
  beta: boolean; // Shows beta string
  print: boolean; // Print Styles and Full Text
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
        <div
          className="container mx-auto px-4 py-4 flex items-center justify-between"
          {...props}
        >
          <div className="flex items-center space-x-4">
            <Link href="/" className="flex items-center space-x-2" prefetch>
              <div className="relative w-10 h-10">
                <Image
                  src={"logo.png"}
                  alt="Hackclub Stade Logo"
                  fill
                  className={`object-contain ${print ? "brightness-0" : ""}`}
                />
              </div>
              <span className="text-xl font-semibold">
                Hackclub Stade - Projektwoche
              </span>
              {beta && (
                <span className="text-xl font-semibold">&lt;Beta&gt;</span>
              )}
              {print && (
                <span className="text-sm font-medium">
                  by The DJL Foundation
                </span>
              )}
            </Link>
          </div>

          {!print && (
            <div className="flex items-center space-x-4">
              <ThemeToggle />
            </div>
          )}
        </div>
      </motion.header>
    </MotionConfig>
  );
}
