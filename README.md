# TailorResume

> **Open browser → Copy job description → Paste → Click Generate → Resume.pdf ready in your folder.**

A cross-platform desktop app that makes applying to jobs effortless. After a one-time setup, your workflow is:

```
┌─────────────────────────────────┐       ┌─────────────────────────────┐
│  Job listing on browser         │       │  TailorResume app           │
│                                 │       │                             │
│  ┌───────────────────────┐      │       │  ┌─────────────────────┐   │
│  │ Software Engineer     │      │  Ctrl │  │ Job Description:    │   │
│  │ at Acme Corp          │──────┼── C ──┼──│                     │   │
│  │                       │      │  ┌┘   │  │ [pasted JD here    │   │
│  │ Requires:             │      │       │  │  .................. ]│   │
│  │ - React, TypeScript   │      │       │  │                     │   │
│  │ - 5+ yrs experience   │      │       │  │ [Generate & Save]   │   │
│  │ - Leadership          │      │       │  │                     │   │
│  └───────────────────────┘      │       │  └─────────────────────┘   │
│                                 │       │                             │
└─────────────────────────────────┘       └─────────────────────────────┘
                                                  │
                                                  ▼
                                        ┌─────────────────────┐
                                        │  Resume.pdf          │
                                        │  ✔ Ready in your     │
                                        │    output folder     │
                                        │  ✔ Previous one      │
                                        │    auto-replaced     │
                                        └─────────────────────┘
```

**No searching for resumes. No renaming files. No manual editing.** Just paste, click, and the tailored PDF instantly lands in your selected folder — the same spot every time, always named `Resume.pdf`, always overwriting the old one.

---

## The Problem It Solves

Applying to jobs usually involves:
1. Finding your latest resume file
2. Wondering which version is the most recent
3. Manually editing or duplicating to avoid overwriting
4. Uploading the wrong file because you have 20 versions

**TailorResume eliminates all of that.** You set your output folder once. Every time you apply:
- Open the job posting on one side of your screen
- Open TailorResume on the other side
- Copy-paste the job description, click **Generate & Save Resume.pdf**
- Switch back to the job application and upload `Resume.pdf` — it's already there, freshly tailored

The previous `Resume.pdf` is automatically replaced. You never have to manage files.

---

## What It Does

TailorResume takes your existing resume (LaTeX or structured profile) and a job description, then uses Google Gemini 2.5 Flash to:

- **Optimize your summary** — rewrites your professional summary to align with the job description's keywords and tone
- **Optimize your skills** — reorders and rephrases your skill sections to highlight what the job actually asks for
- **Optimize your projects** — AI ranks all your projects by relevance to the JD and picks only the **top 2-3 most relevant** ones, rewriting their descriptions to match the job's language
- **Tailor every section** — work experience bullet points, education highlights, certifications, and awards are all rewritten to emphasize what matters for *this* job

The result is a tailored LaTeX document compiled into `Resume.pdf`, saved to your chosen output folder — always the same name, always overwritten.

---

## Why LaTeX? (Recommended Method)

**LaTeX import is the recommended way to use TailorResume.** Here's why:

1. **Your formatting is preserved** — TailorResume keeps your original LaTeX template (columns, fonts, spacing, icons) and only rewrites the *content*. Your resume looks exactly like you designed it, just with smarter text.

2. **More detail = better results** — The more content you put in your imported LaTeX (projects, bullet points, skills, certifications, awards, coursework), the more material the AI has to work with when picking the best 2-3 projects and tailoring each section. A sparse resume gives the AI nothing to optimize.

3. **No reformatting needed** — Unlike the structured Profile mode (which fills a built-in template), LaTeX import keeps your exact layout. You don't lose your custom design.

4. **ATS-friendly** — LaTeX resumes are plain-text based and parse cleanly through Applicant Tracking Systems.

### Tips for best results with LaTeX import

- Include **as many projects as you can** (5-10+) with detailed bullet points. The AI will rank them and pick the best 2-3 for each job.
- Add **all your skills** across multiple categories. The AI will reorder and highlight the relevant ones.
- Write **detailed work experience** with 3-5 bullet points per role. More content = more to optimize.
- Include **certifications, awards, and coursework** — these can be selectively included or dropped based on the JD.

---

## Quick Start

### Prerequisites

- **Node.js** 18+ ([download](https://nodejs.org/))
- **Rust** toolchain ([install via rustup](https://rustup.rs/))
- **Google Gemini API key** — get one free at [Google AI Studio](https://aistudio.google.com/apikey)

### Run from source

```bash
# 1. Clone the repo
git clone https://github.com/YOUR_USERNAME/tailor-resume.git
cd tailor-resume

# 2. Install dependencies
npm install

# 3. Run in dev mode
npm start
# or: npx tauri dev
```

### Build a production installer

```bash
# Build the frontend + Rust backend and produce an installer
npx tauri build --bundles nsis
```

The installer will be at `src-tauri/target/release/bundle/nsis/TailorResume_*.exe`.

---

## How to Use

1. **Add your API key** — On first launch, go to **Settings** and paste your Google Gemini API key. It's stored securely in your OS keychain.

2. **Set your output folder** — In Settings, choose where `Resume.pdf` should be saved. This becomes your "one-click apply" folder. If a file with that name already exists, it will be replaced.

3. **Import your resume (recommended)** — Go to the **Import** tab and paste your existing LaTeX resume. This stores it as "My Original LaTeX" and becomes a template option in the Generate tab. The more content you include, the better the AI can tailor it.

   *Alternative:* Use the **Profile** tab to manually enter your resume data (name, experience, projects, skills, education). This uses a built-in ATS-friendly template instead of your own LaTeX.

4. **Generate a tailored resume** — Go to the **Generate** tab:
   - Paste the **job description** into the text area
   - (Optional) Add **considerations** — specific things you want the AI to include or avoid (e.g., "emphasize leadership", "don't mention frontend")
   - Select a **template**:
     - **ATS Classic** — clean, minimal, single-column
     - **ATS Modern** — clean with subtle section styling
     - **My Original LaTeX** — your imported LaTeX (recommended)
   - Click **Generate & Save Resume.pdf**

5. **Open or preview the result** — After compilation, click **Open** to view the PDF, or **Show in Folder** to reveal it in your file explorer. You can also click **Preview LaTeX Only** to see/edit the generated LaTeX before compiling.

6. **History** — Every generation is saved. View past generations in the **History** tab.

---

## Compile Modes

TailorResume can compile LaTeX to PDF two ways:

| Mode | Requires | How it works |
|------|----------|-------------|
| **Online (default)** | Internet connection | Sends your LaTeX to [latexonline.cc](https://latexonline.cc) (free, open source). Your Gemini API key is **never** sent there. |
| **Offline** | TinyTeX installed (~300MB, one-time download) | Compiles locally with `pdflatex`. Works without internet. Full LaTeX power. |

Switch modes in **Settings → LaTeX Compiler Mode**.

---

## AI Model

Default model is **Gemini 2.5 Flash** (free tier, fast, smart). You can change it in Settings:

- **Gemini 2.5 Flash** — Recommended. Best balance of speed and quality.
- **Gemini 2.5 Flash Lite** — Lighter and faster, good for simple tasks.
- **Gemini 2.0 Flash** — Older but stable, great fallback.
- **Gemini 2.0 Flash Lite** — Lightest model, last resort.
- **Gemini 2.5 Pro** — Most capable but slower. Requires paid billing enabled on your Google account.

If the selected model is overloaded (HTTP 503/429), TailorResume automatically falls back through the chain: 2.5 Flash → 2.0 Flash → 2.5 Flash Lite → 2.0 Flash Lite.

---

## Tech Stack

- **Backend:** Rust + [Tauri v2](https://v2.tauri.app)
- **Frontend:** React 19 + TypeScript + Tailwind CSS + Zustand
- **AI:** Google Gemini API (`generativelanguage.googleapis.com`)
- **LaTeX compilation:** latexonline.cc (online) or TinyTeX/pdflatex (offline)
- **Key storage:** OS keychain via [keyring](https://crates.io/crates/keyring)

---

## Project Structure

```
tailor-resume/
├── src/                        # React frontend
│   ├── components/             # UI components (Settings, Profile, Generate, History, etc.)
│   ├── store/                  # Zustand state management
│   ├── types.ts                # TypeScript interfaces
│   └── App.tsx                 # Root component
├── src-tauri/
│   ├── src/
│   │   ├── gemini.rs           # Gemini API client, prompts, model fallback
│   │   ├── latex.rs            # LaTeX compilation (online + offline), package stripping
│   │   ├── tinytex.rs          # TinyTeX detection, installation, package management
│   │   ├── profile.rs          # Settings + Profile structs, persistence
│   │   ├── templates.rs        # Built-in ATS templates
│   │   ├── history.rs          # Generation history persistence
│   │   ├── keystore.rs         # API key storage (keychain + settings fallback)
│   │   ├── commands.rs         # Tauri command handlers
│   │   └── lib.rs              # App entry, plugin registration
│   ├── capabilities/           # Tauri permissions config
│   └── tauri.conf.json         # Tauri build config
└── package.json
```

---

## Privacy

- Your **Gemini API key** is stored in your OS keychain and is **never** sent anywhere except Google's API.
- When using **online compile mode**, your LaTeX resume content is sent to `latexonline.cc` for compilation. No API keys are included.
- When using **offline compile mode**, everything stays on your machine.
- Your resume data is stored locally in your app config folder. Nothing is uploaded to any server (other than Google's Gemini API for generation).

---

## License

MIT License — see [LICENSE](LICENSE) file.

You are free to use, modify, distribute, and sell this software.

---

## Contributing

Contributions are welcome! To contribute:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## Acknowledgments

- [Tauri](https://tauri.app) — cross-platform app framework
- [Google Gemini](https://deepmind.google/technologies/gemini/) — AI generation
- [latexonline.cc](https://latexonline.cc) — free online LaTeX compiler
- [TinyTeX](https://yihui.org/tinytex/) — lightweight LaTeX distribution