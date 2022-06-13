/**
 * Finalise build process by preparing non-JS components for publishing.
 *
 * This also removes the private: true flag upon copying package.json so that publishing isn't blocked,
 * such config prevents people from publishing the incorrect package.
 *
 * @package: HoloREA
 * @since:   2020-01-31
 */

import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

(async () => {
  fs.copyFileSync(
    path.resolve(__dirname, "./README.md"),
    path.resolve(__dirname, "./build/README.md")
  );
  const packageJson = (
    await import(path.resolve(__dirname, "./package.json"), {
      assert: { type: "json" },
    })
  ).default;
  delete packageJson.scripts["prepare"];
  fs.writeFileSync(
    path.resolve(__dirname, "./build/package.json"),
    JSON.stringify(packageJson, undefined, "  ")
  );
})();
