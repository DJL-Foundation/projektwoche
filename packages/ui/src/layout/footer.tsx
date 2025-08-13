"use client";

import { motion, MotionConfig } from "motion/react";
import Link from "next/link";
import React from "react";
import { GitHub, Mail } from "react-feather";
import { Navigation, Shield } from "lucide-react";

interface FooterProps extends React.HTMLAttributes<HTMLDivElement> {
  beta?: boolean; // Shows beta badge
  print?: boolean; // Print Styles and Full Text
}

export default function Footer({
  beta = false,
  print = false,
  ...props
}: FooterProps) {
  return (
    <MotionConfig reducedMotion={print ? "always" : "user"}>
      <motion.footer
        className={`border-t border-border bg-gradient-to-r from-[#FF8C37]/20 to-[#EC3750]/20 py-8 ${print ? "bg-white text-black" : ""}`}
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={!print ? { duration: 0.5, delay: 0.2 } : {}}
      >
        <div className="container mx-auto px-4" {...props}>
          <div className="flex flex-col items-center justify-between gap-4 md:flex-row">
            <motion.div
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              transition={!print ? { duration: 0.5, delay: 0.3 } : {}}
            >
              <p
                className={`font-medium ${print ? "text-black" : "text-foreground"}`}
              >
                &ldquo;Nachhaltige Webentwicklung&rdquo; - a project by Hackclub
                Stade
              </p>
              <p
                className={`text-sm ${print ? "text-black" : "text-foreground"}`}
              >
                Made with ❤️ for Students by Students
              </p>
            </motion.div>

            {beta && (
              <motion.div
                initial={{ opacity: 0, y: -10 }}
                animate={{ opacity: 1, y: 0 }}
                transition={!print ? { duration: 0.5, delay: 0.3 } : {}}
                className="mx-auto flex items-center justify-center gap-2 rounded bg-yellow-100 px-3 py-1 text-yellow-800 shadow-md"
              >
                <span className="font-bold uppercase">Beta</span>
                <span className="text-xs">
                  This is not public software — Confidential Beta Release.
                </span>
              </motion.div>
            )}

            <motion.div
              className="flex items-center gap-6"
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              transition={!print ? { duration: 0.5, delay: 0.4 } : {}}
            >
              {!print ? (
                <>
                  {/* Normal links - hidden when printing */}
                  <Link
                    href="/terms"
                    prefetch
                    className="text-foreground transition-colors hover:text-primary"
                  >
                    <Navigation className="h-5 w-5" />
                    <span className="sr-only">Terms</span>
                  </Link>
                  <Link
                    href="/privacy"
                    prefetch
                    className="text-foreground transition-colors hover:text-primary"
                  >
                    <Shield className="h-5 w-5" />
                    <span className="sr-only">Privacy</span>
                  </Link>
                  <Link
                    href="https://github.com/djl-foundation/projektwoche"
                    className="text-foreground transition-colors hover:text-primary"
                  >
                    <GitHub className="h-5 w-5" />
                    <span className="sr-only">GitHub</span>
                  </Link>
                  <Link
                    href="mailto:projektwoche@djl.foundation"
                    className="text-foreground transition-colors hover:text-primary"
                  >
                    <Mail className="h-5 w-5" />
                    <span className="sr-only">Email</span>
                  </Link>
                </>
              ) : (
                <>
                  {/* Print-only links with text */}
                  <Link
                    href="https://github.com/djl-foundation/projektwoche"
                    className=" transition-colors hover:text-primary text-blue-700 no-underline"
                  >
                    <GitHub className="inline-block h-5 w-5" />
                    <span className="ml-1">@djl-foundation</span>
                  </Link>
                  <Link
                    href="mailto:projektwoche@djl.foundation"
                    className=" transition-colors hover:text-primary text-blue-700 no-underline"
                  >
                    <Mail className="inline-block h-5 w-5" />
                    <span className="ml-1">projektwoche@djl.foundation</span>
                  </Link>
                </>
              )}
            </motion.div>
          </div>

          <motion.div
            className={`mt-6 border-t border-border/30 pt-6 text-center text-sm ${print ? "text-black" : "text-foreground"}`}
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={!print ? { duration: 0.5, delay: 0.5 } : {}}
          >
            <motion.div
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              transition={!print ? { duration: 0.5, delay: 0.35 } : {}}
            >
              <p
                className={`pt-1 text-center text-sm ${print ? "text-black" : "text-foreground"}`}
              >
                Hackclub Stade does not endorse any projects hosted by students
                on our platform.
              </p>
            </motion.div>
            <p>
              © {new Date().getFullYear()} By The DJL Foundation. All rights
              reserved.
            </p>
          </motion.div>
        </div>
      </motion.footer>
    </MotionConfig>
  );
}
