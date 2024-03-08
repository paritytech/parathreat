# Parathreat

## Introduction

Parathreat is a wargame centered on securing the Polkadot SDK ecosystem, offering an array of missions for participants to navigate and conquer. With the initial mission ready for engagement and more on the horizon, the game covers extensive aspects of the Polkadot ecosystem, including parachains, pallets, and nodes within the Substrate framework. Participants are tasked with identifying and rectifying security vulnerabilities across these diverse components, mirroring the real-world scenarios of threat detection and resolution.

### Gameplay Modes

This wargame is designed with two distinct gameplay modes, each emphasizing a different facet of cybersecurity within the Polkadot SDK environment:

- **Red Team Mode:** Tailored for those seeking an intensive challenge, this mode equips players with targets and basic exploits. The goal is to customize and deploy these exploits against the provided targets. Success in Red Team mode requires not only discovering vulnerabilities but also creatively leveraging them, reflecting the tactics of attackers intent on compromising network defenses.

- **Blue Team Mode:** Offering a slightly more approachable level of difficulty, Blue Team mode presents players with the same targets alongside active threat scripts that are currently exploiting these targets. Players must analyze these attacks to pinpoint vulnerabilities and develop effective countermeasures, adopting the role of defenders tasked with protecting the network against threats.

## What You Can Expect to Learn

Parathreat equips you with insights and skills relevant to the Polkadot SDK ecosystem:

- **Understanding Polkadot SDK Security Risks:** Gain a comprehensive understanding of potential security risks within the Polkadot SDK, including common vulnerabilities that could compromise parachains and the broader Substrate framework.
- **Identifying Vulnerabilities:** Learn how to meticulously scan and identify weaknesses within the Polkadot ecosystem, utilizing systematic approaches to uncover potential threats.
- **Exploiting and Mitigating Threats:** Master the art of exploiting identified vulnerabilities to understand their implications, followed by developing and implementing robust mitigation strategies to safeguard the ecosystem.
- **Enhancing Development and Auditing Skills:** Whether you aspire to excel as a developer or auditor within the Polkadot ecosystem, Parathreat offers a hands-on learning experience to refine your abilities, ensuring you are well-equipped to contribute to the security and integrity of the Polkadot SDK.

## Getting Started

Ideal for individuals with foundational knowledge of the Substrate framework and an interest in cybersecurity, Parathreat challenges you to think both offensively and defensively, enhancing your proficiency in securing the Polkadot ecosystem. If you are new to the Substrate framework, consider familiarizing yourself with the basics before embarking on this wargame, try [Polkadot Blockchain Academy's Book](https://github.com/Polkadot-Blockchain-Academy/pba-book) or the [Substrate Developer Hub](https://github.com/substrate-developer-hub).

## Missions

### Casino

The first mission in Parathreat, Casino, is now available for engagement. This mission is designed to test your skills in identifying and exploiting vulnerabilities within Polkadot SDK v1.3.0. You need to analyze the provided targets and develop exploits to compromise them, or alternatively, defend against active threats. The targets are:

1. **Casino Parachain:** A parachain runtime using the Raffle, Poker and Slots pallets. [Code](targets/runtimes/casino/).
2. **Raffle Pallet:** A pallet that allows users to participate in a raffle each time they play in the casino. [Code](targets/pallets/raffle/).
3. **Poker Pallet:** A pallet that allows users to play poker in the casino. [Code](targets/pallets/poker/).
4. **Slots Pallet:** A pallet that allows users to play slots in the casino. [Code](targets/pallets/slots/).

The first week, you can only play in Red Team mode. To write a need exploit you can follow the [Raffle E2E Test](threats/src/casino/raffle.rs) and the [Raffle Exploit Template](threats/src/casino/raffle_exploit_template.rs), always write your tests in [`Casino Threats path`](threats/src/casino/). Ensure you're running the exploit tests in `--release` mode (to simulate a closer scenario to reality), for this is suggested to use `cargo-nextest` or just `cargo run`. If you think you have exploited all the vulnerabilities (7 for this Casino), feel free to contact us to verify your findings. After a week, the second phase will start, and you will be able to play in Blue Team mode. This means that all the exploit are going to be revealed and you will be able to compare with your exploits.

The second one an so on, you can play in Blue Team mode, just run the tests with `cargo run -- -t blue` to check how many vulnerabilities are still exploitable, try to catch them, fix them and once you have fixed all the vulnerabilities, you win!

### More Missions

More missions are coming soon, stay tuned for updates on new missions and their respective targets. Feel free to contribute to the development of new missions and targets, as well as the enhancement of existing ones. We welcome your participation in making Parathreat a comprehensive and engaging wargame for the Polkadot SDK ecosystem.
