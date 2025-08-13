"use client";

import { useState, useEffect, useRef } from "react";
import { useTheme } from "next-themes";
import type React from "react";
import { VercelToolbar } from "@vercel/toolbar/next";
import Header from "./layout/header";
import Footer from "./layout/footer";
import { Toaster } from "./ui/sonner";

interface LayoutContentProps {
  children: React.ReactNode;
  shouldShowVercelToolbar: boolean;
  beta: boolean;
}

export default function LayoutContent({
  children,
  shouldShowVercelToolbar,
  beta,
}: LayoutContentProps) {
  const [isPrintMode, setIsPrintMode] = useState(false);
  const { setTheme, theme } = useTheme();
  const originalThemeRef = useRef<string | undefined>(undefined);

  useEffect(() => {
    const handleBeforePrint = () => {
      originalThemeRef.current = theme;
      setIsPrintMode(true);
      setTheme("light");
      setTimeout(() => {
        console.log("Print styles applied");
      }, 100);
    };

    const handleAfterPrint = () => {
      setIsPrintMode(false);
      setTheme(originalThemeRef.current ?? "system");
    };

    const handleKeyDown = (event: KeyboardEvent) => {
      // Check for Ctrl+P or Cmd+P
      if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "p") {
        event.preventDefault();
        originalThemeRef.current = theme;
        setIsPrintMode(true);
        setTheme("light");
        
        // Create one-time print completion handler
        const onAfterPrint = () => {
          setIsPrintMode(false);
          setTheme(originalThemeRef.current ?? "system");
          window.removeEventListener("afterprint", onAfterPrint);
        };
        
        window.addEventListener("afterprint", onAfterPrint);
        
        setTimeout(() => {
          console.log("Print styles applied");
          window.print();
        }, 100);
      }
    };

    window.addEventListener("beforeprint", handleBeforePrint);
    window.addEventListener("afterprint", handleAfterPrint);
    window.addEventListener("keydown", handleKeyDown);

    return () => {
      window.removeEventListener("beforeprint", handleBeforePrint);
      window.removeEventListener("afterprint", handleAfterPrint);
      window.removeEventListener("keydown", handleKeyDown);
    };
  }, [theme, setTheme]);

  return (
    <>
      <Toaster />
      <div className="min-h-screen flex flex-col bg-background text-foreground">
        <Header print={isPrintMode} beta={beta} />
        <main className="grow">{children}</main>
        <Footer print={isPrintMode} beta={beta} />
      </div>
      {shouldShowVercelToolbar && <VercelToolbar />}
    </>
  );
}
