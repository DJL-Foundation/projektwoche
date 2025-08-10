import NextLink, { type LinkProps as NextLinkProps } from "next/link";
import { type AnchorHTMLAttributes } from "react";

interface LinkProps
  extends Omit<AnchorHTMLAttributes<HTMLAnchorElement>, "href">,
    Omit<NextLinkProps, "href"> {
  href: string | URL; // Unified href type
}

export default function Link({ href, children, ...props }: LinkProps) {
  const runningNext: boolean =
    typeof window !== "undefined" &&
    !!(window as unknown as { next: unknown }).next;

  if (!runningNext) {
    return (
      <a href={href.toString()} {...props}>
        {children}
      </a>
    );
  }

  return (
    <NextLink href={href} {...props}>
      {children}
    </NextLink>
  );
}
