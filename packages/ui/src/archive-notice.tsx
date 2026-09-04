import { Archive, Database, ServerOff } from "lucide-react";

export function ArchiveNotice() {
  return (
    <aside
      className="border-b border-amber-400/30 bg-amber-50 text-amber-950 dark:bg-amber-950/40 dark:text-amber-100"
      aria-label="Archivhinweis"
    >
      <div className="container mx-auto flex flex-col gap-3 px-4 py-4 md:flex-row md:items-center md:justify-between">
        <div className="flex gap-3">
          <Archive className="mt-0.5 h-5 w-5 shrink-0" aria-hidden="true" />
          <div>
            <p className="font-semibold">Archivierte Projektdokumentation</p>
            <p className="max-w-3xl text-sm leading-relaxed opacity-90">
              Dieses Projekt wird nicht mehr aktiv betrieben. Die Oberfläche
              bleibt zu Dokumentations- und historischen Zwecken erhalten;
              interaktive Funktionen können eingeschränkt sein.
            </p>
          </div>
        </div>
        <div className="flex shrink-0 flex-wrap gap-2 text-xs font-medium">
          <span className="inline-flex items-center gap-1 rounded-full border border-current/20 px-2.5 py-1">
            <ServerOff className="h-3.5 w-3.5" aria-hidden="true" />
            Backend eingestellt
          </span>
          <span className="inline-flex items-center gap-1 rounded-full border border-current/20 px-2.5 py-1">
            <Database className="h-3.5 w-3.5" aria-hidden="true" />
            Externe Dienste ggf. offline
          </span>
        </div>
      </div>
    </aside>
  );
}
