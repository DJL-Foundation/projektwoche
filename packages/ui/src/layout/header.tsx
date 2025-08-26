import type React from "react";
import Link from "next/link";
import Image from "next/image";
import { ThemeToggle } from "../theme-toggle";
import { motion, MotionConfig } from "motion/react";
import { Button } from "../ui/button";
import { ArrowLeft } from "lucide-react";
import { useEffect, useState } from "react";

interface ProjectsData {
  activeYear: number;
  workshops: Record<string, {
    '!'?: { displayName: string; site?: string };
    [username: string]: any;
  }>;
}

interface HeaderProps extends React.HTMLAttributes<HTMLDivElement> {
  beta?: boolean; // Shows beta string
  print?: boolean; // Print Styles and Full Text
}

async function fetchProjectsData(): Promise<ProjectsData | null> {
  try {
    const response = await fetch('/api/projekte/data', {
      headers: {
        'authorization': 'prowo-will-implement-security'
      }
    });
    
    if (!response.ok) {
      console.error('Failed to fetch projects data:', response.status);
      return null;
    }
    
    return await response.json();
  } catch (error) {
    console.error('Error fetching projects data:', error);
    return null;
  }
}

function getLatestWorkshops(data: ProjectsData, count: number = 3): Array<{ year: number; displayName: string }> {
  const years = Object.keys(data.workshops)
    .map(year => parseInt(year, 10))
    .sort((a, b) => b - a);
  
  return years.slice(0, count).map(year => ({
    year,
    displayName: data.workshops[year.toString()]?.['!']?.displayName || `Projektwoche ${year}`
  }));
}

interface HeaderProps extends React.HTMLAttributes<HTMLDivElement> {
  beta?: boolean; // Shows beta string
  print?: boolean; // Print Styles and Full Text
}

export default function Header({
  beta = false,
  print = false,
  ...props
}: HeaderProps) {
  const [projectsData, setProjectsData] = useState<ProjectsData | null>(null);
  const [latestWorkshops, setLatestWorkshops] = useState<Array<{ year: number; displayName: string }>>([]);

  useEffect(() => {
    fetchProjectsData().then(data => {
      if (data) {
        setProjectsData(data);
        setLatestWorkshops(getLatestWorkshops(data, 3));
      }
    });
  }, []);

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

              {/* Right section - Workshop navigation */}
              <div className="flex justify-end items-center space-x-2">
                {latestWorkshops.map(({ year, displayName }) => (
                  <Button
                    key={year}
                    variant={year === projectsData?.activeYear ? "default" : "outline"}
                    asChild
                    size="sm"
                  >
                    <Link href={`/projekte/${year}`}>
                      {year}
                    </Link>
                  </Button>
                ))}
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
