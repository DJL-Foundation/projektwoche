import { ArchiveOverview } from "prowo-ui/archive-overview";

export default function AboutPage() {
  return (
    <div className="py-12">
      <div className="container mx-auto max-w-4xl px-4">
        <p className="text-primary mb-3 text-sm font-semibold tracking-wide uppercase">
          Über das Archiv
        </p>
        <h1 className="mb-6 text-4xl font-bold">Nachhaltige Webentwicklung</h1>
        <div className="text-muted-foreground space-y-5 text-lg leading-relaxed">
          <p>
            Das Projekt begleitete die Projektwoche 2025 am Gymnasium Athenaeum
            Stade. Schülerinnen und Schüler lernten Grundlagen der
            Webentwicklung und setzten eigene Websites sowie Ideen rund um
            nachhaltigere IT um.
          </p>
          <p>
            Das Monorepo verband eine Next.js-Startseite, eine Astro-Galerie für
            Schülerprojekte, gemeinsame UI-Komponenten und ein in Rust
            geschriebenes Setup-Werkzeug. Die damalige Bereitstellung setzte auf
            Vercel-Microfrontends, GitHub Pages und optionale Analytics.
          </p>
          <p>
            Heute ist die Anwendung ein bewusst ehrliches Archiv: Navigation und
            repräsentative Projektflüsse bleiben erhalten, während nicht mehr
            betriebene Backend-, Analytics- und Vorschau-Dienste klar als
            eingestellt behandelt werden.
          </p>
        </div>
      </div>
      <div className="mt-12">
        <ArchiveOverview />
      </div>
    </div>
  );
}
