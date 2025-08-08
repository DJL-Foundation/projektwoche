import Link from "next/link";

export default function Page() {
  return (
    <div className="flex h-full w-full items-center justify-center">
      This page is currently under development, to access download of the cli
      please visit the repository at{" "}
      <Link href="https://github.com/djl-foundation/projektwoche">
        https://github.com/djl-foundation/projektwoche
      </Link>
    </div>
  );
}
