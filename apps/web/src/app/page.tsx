import type { Metadata } from "next";
import Unauthorized from "./unauthorised";

export const metadata: Metadata = {
  title: "Presentation Foundation",
  description: "This Metadata should be inaccessible due to middleware",
};

export default Unauthorized;
