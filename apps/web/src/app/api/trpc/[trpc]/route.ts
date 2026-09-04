const retiredResponse = () =>
  Response.json(
    {
      status: "archived",
      message: "The original tRPC backend is no longer running.",
    },
    { status: 410 },
  );

export const GET = retiredResponse;
export const POST = retiredResponse;
