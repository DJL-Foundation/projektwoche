import type React from "react";
import { useEffect, useState } from "react";
import Link from "next/link";
import Image from "next/image";
import { ThemeToggle } from "../theme-toggle";
import { motion, MotionConfig } from "motion/react";
import { Button } from "../ui/button";
import { ArrowLeft } from "lucide-react";
import projectsData from "../../../../projects.json";

interface ProjectsData {
  activeYear: number;
  workshops: Record<
    string,
    {
      "!"?: { displayName: string; site?: string };
      [username: string]: unknown;
    }
  >;
}

interface HeaderProps extends React.HTMLAttributes<HTMLDivElement> {
  beta?: boolean; // Shows beta string
  print?: boolean; // Print Styles and Full Text
}

function getLatestWorkshops(
  data: ProjectsData,
  count = 3,
): Array<{ year: number; displayName: string }> {
  const years = Object.keys(data.workshops)
    .map((year) => parseInt(year, 10))
    .sort((a, b) => b - a);

  return years.slice(0, count).map((year) => ({
    year,
    displayName:
      data.workshops[year.toString()]?.["!"]?.displayName ??
      `Projektwoche ${year}`,
  }));
}

export default function Header({
  beta = false,
  print = false,
  ...props
}: HeaderProps) {
  const [latestWorkshops, setLatestWorkshops] = useState<
    Array<{ year: number; displayName: string }>
  >([]);

  useEffect(() => {
    // Use the imported data directly instead of fetching
    const data = projectsData as ProjectsData;
    setLatestWorkshops(getLatestWorkshops(data, 3));
  }, []);

  return (
    <MotionConfig reducedMotion={print ? "always" : "user"}>
      <motion.header
        className={`bg-background ${print ? "border-b-2" : "border-border border-b"}`}
        initial={{ opacity: 0, y: -20 }}
        animate={{ opacity: 1, y: 0 }}
      >
        <div className="container mx-auto px-4 py-4" {...props}>
          {!print ? (
            <div className="grid grid-cols-3 items-center">
              {/* Left section - Hackclub Stade button */}
              <div className="flex items-center space-x-4">
                <Button
                  variant="ghost"
                  asChild
                  disabled
                  className="cursor-not-allowed"
                >
                  <a
                    href="https://hackclub-stade.de"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="flex items-center gap-2"
                  >
                    <ArrowLeft className="h-4 w-4" />
                    Hackclub Stade
                  </a>
                </Button>
              </div>

              {/* Center section - Logo and title */}
              <div className="flex justify-center">
                <Link href="/" className="flex items-center space-x-2" prefetch>
                  <div className="relative h-10 w-10">
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

              {/* Right section - Workshop navigation */}
              <div className="flex items-center justify-end space-x-2">
                {latestWorkshops.map(({ year, displayName: _displayName }) => (
                  <div key={year} className="flex items-center space-x-1">
                    <Button
                      variant={
                        year === (projectsData as ProjectsData).activeYear
                          ? "default"
                          : "outline"
                      }
                      asChild
                      size="sm"
                    >
                      <Link href={`/projekte/${year}`}>{year}</Link>
                    </Button>
                    <Button
                      variant="outline"
                      asChild
                      size="sm"
                      className="px-2"
                      title={`Slideshow ${year}`}
                    >
                      <Link href={`/projekte/${year}/slideshow`}>▶</Link>
                    </Button>
                  </div>
                ))}
                <ThemeToggle />
              </div>
            </div>
          ) : (
            // Print layout remains the same
            <div className="flex items-center justify-between">
              <div className="flex items-center space-x-4">
                <Link href="/" className="flex items-center space-x-2" prefetch>
                  <div className="relative h-10 w-10">
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
