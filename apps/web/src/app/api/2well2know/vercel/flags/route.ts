export function GET() {
  return Response.json(
    { status: "archived", message: "Feature flag discovery is retired." },
    { status: 410 },
  );
}
