import Link from "next/link";
import { Button } from "./ui/button";
import { ArrowRight, Code, Leaf } from "lucide-react";

export function HeroSection() {
  return (
    <section className="relative flex min-h-screen items-center justify-center overflow-hidden bg-transparent">
      {/* Background Pattern */}
      <div className="absolute inset-0 opacity-5">
        <div className="border-primary absolute top-20 left-20 h-32 w-32 rounded-full border-2"></div>
        <div className="border-accent absolute top-40 right-32 h-24 w-24 rounded-full border-2"></div>
        <div className="border-secondary absolute bottom-32 left-1/4 h-16 w-16 rounded-full border-2"></div>
        <div className="border-primary absolute right-20 bottom-20 h-20 w-20 rounded-full border-2"></div>
      </div>

      <div className="relative z-10 container mx-auto px-4 text-center">
        <div className="mx-auto max-w-4xl">
          {/* Icons */}
          <div className="mb-8 flex justify-center gap-4">
            <div className="bg-primary/10 rounded-full p-3">
              <Code className="text-primary h-8 w-8" />
            </div>
            <div className="bg-accent/10 rounded-full p-3">
              <Leaf className="text-accent h-8 w-8" />
            </div>
          </div>

          {/* Main Heading */}
          <h1 className="mb-6 text-4xl leading-tight font-bold text-balance md:text-6xl">
            Innovationen für eine{" "}
            <span className="text-primary">nachhaltige</span> Zukunft
          </h1>

          {/* Subheading */}
          <p className="text-muted-foreground mx-auto mb-8 max-w-3xl text-xl leading-relaxed text-pretty md:text-2xl">
            Entdecke die Projektwoche &quot;Nachhaltige Webentwicklung&quot; am
            Gymnasium Athenaeum Stade. Schüler entwickeln innovative Webseiten
            mit Fokus auf Umweltbewusstsein und moderne Technologien.
          </p>

          {/* CTA Buttons */}
          <div className="flex flex-col items-center justify-center gap-4 sm:flex-row">
            <Button size="lg" className="group px-8 py-6 text-lg" asChild>
              <Link href="/projekte" prefetch>
                Projekte entdecken
                <ArrowRight className="ml-2 h-5 w-5 transition-transform group-hover:translate-x-1" />
              </Link>
            </Button>
            <Button
              variant="outline"
              size="lg"
              className="bg-transparent px-8 py-6 text-lg"
              asChild
            >
              <Link href="/projekte/slideshow" prefetch>
                ▶ Slideshow
              </Link>
            </Button>
            <Button
              variant="outline"
              size="lg"
              className="bg-transparent px-8 py-6 text-lg"
              asChild
            >
              <Link href="/about" prefetch>
                Über das Projekt
              </Link>
            </Button>
          </div>

          {/* Stats */}
          <div className="mx-auto mt-16 grid max-w-2xl grid-cols-3 gap-8">
            <div className="text-center">
              <div className="text-primary mb-2 text-3xl font-bold">8</div>
              <div className="text-muted-foreground text-sm">Schüler</div>
            </div>
            <div className="text-center">
              <div className="text-accent mb-2 text-3xl font-bold">16</div>
              <div className="text-muted-foreground text-sm">Projekte</div>
            </div>
            <div className="text-center">
              <div className="text-secondary mb-2 text-3xl font-bold">1</div>
              <div className="text-muted-foreground text-sm">Woche</div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
