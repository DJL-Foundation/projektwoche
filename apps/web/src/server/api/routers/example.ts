import { z } from "zod";

import { createTRPCRouter, publicProcedure } from "~/server/api/trpc";

export const exampleRouter = createTRPCRouter({
  respond: publicProcedure.input(z.string()).query(({ input }) => {
    return {
      response: `Input ${input}`,
    };
  }),
});
