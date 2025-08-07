interface Program {
  name: string;
  assert_available: string;
}

export async function builder(programs: Program[]): Promise<void> {
  console.log("Verifying installation...");

  let allPassed = true;

  for (const program of programs) {
    try {
      // Check if command is available
      const proc = Bun.spawn([program.assert_available, "--version"], {
        stdout: "pipe",
        stderr: "pipe",
      });

      const exitCode = await proc.exited;

      if (exitCode === 0) {
        const version = await new Response(proc.stdout).text();
        console.log(`✅ ${program.name} is installed`);
        console.log(`   Version: ${version.trim()}`);
      } else {
        console.log(`❌ ${program.name} is not installed`);
        allPassed = false;
      }
    } catch (error) {
      console.log(`❌ ${program.name} is not installed`);
      allPassed = false;
    }
  }

  if (allPassed) {
    console.log("All components verified successfully!");
    process.exit(0);
  } else {
    console.log("Some components failed verification!");
    process.exit(1);
  }
}
