[vite+]: https://viteplus.dev
[anthropic]: https://anthropic.com
[solarwinds Hack]: https://www.youtube.com/watch?v=Eq6ATHhBezw
[LiteLLM Hack]: https://snyk.io/articles/poisoned-security-scanner-backdooring-litellm/
[@awfixer]: https://github.com/awfixer
[VoidZero]: https://voidzero.dev

## What is Weblang?

Weblang is a security-first fork of [vite+] that eliminates the default yarn/pnpm/Node.js toolchain in favor of a hardened, minimal runtime.

It replaces insecure ecosystem dependencies with a modified Bun-based runtime and package manager (with linking support coming soon). The goal is to sharply reduce cascading supply-chain risks that have repeatedly compromised major projects.

Registry locking is planned: projects will be able to lock to a tightly curated registry, ensuring only vetted packages can be installed or updated — even when agents are used for development or maintenance.

This directly addresses well-known supply-chain attacks such as the [SolarWinds Hack] and the more recent [LiteLLM Hack].

## Credits

- Original inspiration and ecosystem direction drawn from the [VoidZero] / [Vite+] work.
- Modified Bun runtime and package manager (Bun itself was later acquired by [Anthropic]).

## Contributing

Other developers are already assisting. If you would like to contribute, apply to join the Discord server. We will evaluate your background and what you bring to the project.

## Background

I design systems with a deep awareness of how nation-state actors, Advanced Persistent Threats, and access brokers compromise infrastructure. Weblang is built to raise the bar in an AI-accelerated threat landscape.

Cheers.

- [@awfixer]
