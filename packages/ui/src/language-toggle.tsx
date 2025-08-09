"use client";

import { useState, useEffect } from "react";
import { Globe } from "react-feather";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "#ui/select";

interface LanguageToggleProps {
  onChange: (language: "de" | "en") => void;
  initialLanguage?: "de" | "en";
}

export function LanguageToggle({
  onChange,
  initialLanguage = "de",
}: LanguageToggleProps) {
  const [language, setLanguage] = useState<"de" | "en">(initialLanguage);

  useEffect(() => {
    // Check if there's a stored preference
    const storedLanguage = localStorage.getItem("preferredLanguage") as
      | "de"
      | "en"
      | null;
    if (
      storedLanguage &&
      (storedLanguage === "de" || storedLanguage === "en")
    ) {
      setLanguage(storedLanguage);
    }
  }, []);

  useEffect(() => {
    // Save preference and notify parent
    localStorage.setItem("preferredLanguage", language);
    onChange(language);
  }, [language, onChange]);

  return (
    <div className="flex items-center gap-2">
      <Globe className="h-4 w-4 text-muted-foreground" />
      <Select
        value={language}
        onValueChange={(value) => setLanguage(value as "de" | "en")}
      >
        <SelectTrigger className="w-[120px]">
          <SelectValue placeholder="Language" />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="de">Deutsch</SelectItem>
          <SelectItem value="en">English</SelectItem>
        </SelectContent>
      </Select>
    </div>
  );
}
