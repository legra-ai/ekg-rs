"""Dependabot merges must use the dedicated App without release authority."""

import unittest
from pathlib import Path


class DependabotPolicyTests(unittest.TestCase):
    def test_formatter_pin_matches_local_hooks(self):
        root = Path(__file__).resolve().parents[1]
        workflow = (root / ".github/workflows/ci.yml").read_text()
        formatter = workflow.split("  fmt:", 1)[1].split("  clippy:", 1)[0]
        self.assertIn("nightly-2026-10-16", formatter)
        self.assertIn("nightly-2026-10-16", (root / ".githooks/pre-commit").read_text())

    def test_dedicated_merge_identity(self):
        root = Path(__file__).resolve().parents[1]
        path = root / ".github/workflows/dependabot-merge.yml"
        self.assertTrue(path.exists(), "A trusted completed-CI handler is required")
        workflow = path.read_text()
        self.assertNotIn("secrets.RELEASE_WRITER_APP_", workflow)
        self.assertIn("app-id: ${{ secrets.RELEASE_MERGER_APP_ID }}", workflow)
        self.assertIn("private-key: ${{ secrets.RELEASE_MERGER_APP_KEY }}", workflow)
        action = workflow.split("merge-tested-dependabot@", 1)[1]
        self.assertIn("app-id: ${{ secrets.RELEASE_MERGER_APP_ID }}", action)
        self.assertIn("workflow_run:", workflow)
        self.assertNotIn("pull_request_target:", workflow)


if __name__ == "__main__":
    unittest.main()
