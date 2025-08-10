"use client";

import { useState, useEffect } from "react";
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

  useEffect(() => {
    let originalTheme: string | undefined;

    const handleBeforePrint = () => {
      originalTheme = theme;
      setIsPrintMode(true);
      setTheme("light");
      setTimeout(() => {
        console.log("Print styles applied");
      }, 100);
    };

    const handleAfterPrint = () => {
      setIsPrintMode(false);
      setTheme(originalTheme ?? "system");
    };

    const handleKeyDown = (event: KeyboardEvent) => {
      // Check for Ctrl+P or Cmd+P
      if ((event.ctrlKey || event.metaKey) && event.key === "p") {
        event.preventDefault();
        originalTheme = theme;
        setIsPrintMode(true);
        setTheme("light");
        setTimeout(() => {
          console.log("Print styles applied");
        }, 100);
        window.print();
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
