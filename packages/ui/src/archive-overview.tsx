import {
  Archive,
  BookOpen,
  CalendarDays,
  Github,
  UserRound,
} from "lucide-react";
import { Card, CardContent } from "./ui/card";

const metadata = [
  { icon: Archive, label: "Status", value: "Archiviert" },
  { icon: CalendarDays, label: "Ursprünglicher Zeitraum", value: "2025" },
  { icon: Github, label: "Quelle", value: "GitHub" },
  { icon: UserRound, label: "Maintainer", value: "Jack Ruder" },
];

export function ArchiveOverview() {
  return (
    <section className="bg-muted/30 py-16" aria-labelledby="archive-heading">
      <div className="container mx-auto px-4">
        <div className="mx-auto grid max-w-5xl gap-8 lg:grid-cols-[1.25fr_1fr]">
          <div>
            <div className="text-primary mb-4 flex items-center gap-2 text-sm font-semibold tracking-wide uppercase">
              <BookOpen className="h-4 w-4" aria-hidden="true" />
              Rückblick
            </div>
            <h2 id="archive-heading" className="mb-4 text-3xl font-bold">
              Eine Projektwoche als Lernlabor
            </h2>
            <div className="text-muted-foreground space-y-4 leading-relaxed">
              <p>
                „Nachhaltige Webentwicklung“ führte Schülerinnen und Schüler in
                HTML, CSS und JavaScript ein. Das Ziel war, innerhalb einer
                Woche eine persönliche Website und eine Idee für nachhaltigere
                digitale Technik sichtbar zu machen.
              </p>
              <p>
                Das Archiv bewahrt die damalige Navigation, Projektübersichten
                und repräsentative Ansichten. Es dokumentiert zugleich eine
                wichtige Erkenntnis: Eine robuste Präsentation braucht klare
                Fallbacks, wenn Hosting, Analytics oder andere externe Dienste
                nach Projektende nicht mehr verfügbar sind.
              </p>
            </div>
          </div>

          <Card className="self-start">
            <CardContent className="grid gap-4 p-6 sm:grid-cols-2 lg:grid-cols-1">
              {metadata.map(({ icon: Icon, label, value }) => (
                <div key={label} className="flex items-center gap-3">
                  <div className="bg-primary/10 text-primary rounded-md p-2">
                    <Icon className="h-4 w-4" aria-hidden="true" />
                  </div>
                  <div>
                    <p className="text-muted-foreground text-xs uppercase">
                      {label}
                    </p>
                    <p className="font-medium">{value}</p>
                  </div>
                </div>
              ))}
            </CardContent>
          </Card>
        </div>
      </div>
    </section>
  );
}
