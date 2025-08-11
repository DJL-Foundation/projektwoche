"use client";

import Link from "../lib/link";
import { motion } from "motion/react";
import { Button } from "../ui/button";
import { Shield, Home } from "react-feather";
import { Card, CardContent } from "../ui/card";

export default function Forbidden() {
  return (
    <div className="container mx-auto px-4 py-16 min-h-[80vh] flex items-center justify-center">
      <Card className="max-w-md w-full">
        <CardContent className="pt-6 pb-8 px-6">
          <motion.div
            className="text-center"
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.5 }}
          >
            <motion.div
              initial={{ scale: 0.8, opacity: 0 }}
              animate={{ scale: 1, opacity: 1 }}
              transition={{
                duration: 0.5,
                delay: 0.2,
                type: "spring",
                stiffness: 200,
              }}
              className="mx-auto mb-8 relative"
            >
              <div className="w-32 h-32 bg-red-500/10 rounded-full flex items-center justify-center mx-auto">
                <Shield className="h-16 w-16 text-red-500" />
              </div>
              <motion.div
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                transition={{ delay: 0.6, duration: 0.3 }}
                className="absolute top-0 right-0 w-10 h-10 bg-red-500/20 rounded-full"
                style={{ transform: "translate(25%, -25%)" }}
              />
              <motion.div
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                transition={{ delay: 0.7, duration: 0.3 }}
                className="absolute bottom-0 left-0 w-6 h-6 bg-red-500/15 rounded-full"
                style={{ transform: "translate(-25%, 25%)" }}
              />
            </motion.div>

            <motion.h1
              className="text-3xl font-bold mb-4"
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.4, delay: 0.3 }}
            >
              403 Forbidden
            </motion.h1>

            <motion.p
              className="text-muted-foreground mb-8"
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.4, delay: 0.4 }}
            >
              Access to this resource is forbidden. You do not have the
              necessary permissions to view this content.
            </motion.p>

            <motion.div
              className="flex flex-col sm:flex-row gap-4 justify-center"
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.4, delay: 0.5 }}
            >
              <Button variant="outline" asChild>
                <Link href="/" className="flex items-center gap-2">
                  <Home className="h-4 w-4" />
                  Return Home
                </Link>
              </Button>
            </motion.div>
          </motion.div>
        </CardContent>
      </Card>
    </div>
  );
}
