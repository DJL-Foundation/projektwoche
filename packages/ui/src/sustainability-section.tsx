import { Card, CardContent } from "./ui/card";
import { Leaf, Zap, Globe, Recycle } from "lucide-react";

const sustainabilityFeatures = [
  {
    icon: Leaf,
    title: "Grüne Technologien",
    description:
      "Verwendung umweltfreundlicher Hosting-Lösungen und optimierter Code-Praktiken",
  },
  {
    icon: Zap,
    title: "Energieeffizienz",
    description:
      "Entwicklung von Webseiten mit minimalem Energieverbrauch und schnellen Ladezeiten",
  },
  {
    icon: Globe,
    title: "Globale Verantwortung",
    description:
      "Bewusstsein für die Auswirkungen digitaler Technologien auf unseren Planeten",
  },
  {
    icon: Recycle,
    title: "Nachhaltige Praktiken",
    description:
      "Wiederverwendbare Code-Komponenten und ressourcenschonende Entwicklungsmethoden",
  },
];

export function SustainabilitySection() {
  return (
    <section className="bg-background py-16">
      <div className="container mx-auto px-4">
        <div className="mb-12 text-center">
          <h2 className="mb-4 text-3xl font-bold text-balance">
            Nachhaltigkeit im Fokus
          </h2>
          <p className="text-muted-foreground mx-auto max-w-2xl text-lg text-pretty">
            Unsere Schüler lernen nicht nur Webentwicklung, sondern auch wie
            Technologie verantwortungsvoll und umweltbewusst eingesetzt werden
            kann.
          </p>
        </div>

        <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-4">
          {sustainabilityFeatures.map((feature, index) => (
            <Card
              key={index}
              className="text-center transition-shadow hover:shadow-md"
            >
              <CardContent className="p-6">
                <div className="bg-primary/10 mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-full">
                  <feature.icon className="text-primary h-6 w-6" />
                </div>
                <h3 className="mb-3 text-lg font-semibold text-balance">
                  {feature.title}
                </h3>
                <p className="text-muted-foreground text-sm text-pretty">
                  {feature.description}
                </p>
              </CardContent>
            </Card>
          ))}
        </div>
      </div>
    </section>
  );
}
