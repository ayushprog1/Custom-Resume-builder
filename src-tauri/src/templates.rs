use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub id: String,
    pub name: String,
    pub description: String,
    pub source: String,
    pub built_in: bool,
}

fn ats_classic() -> &'static str {
    r#"\documentclass[11pt,letterpaper]{article}

\usepackage[utf8]{inputenc}
\usepackage[T1]{fontenc}
\usepackage{lmodern}
\usepackage[margin=0.75in,top=0.6in,bottom=0.6in]{geometry}
\usepackage{enumitem}
\usepackage{hyperref}
\usepackage{xcolor}
\usepackage{parskip}

\hypersetup{colorlinks=true, urlcolor=blue, linkcolor=blue}

\setlist[itemize]{leftmargin=*, itemsep=1pt, topsep=2pt, parsep=0pt}

\newcommand{\resumesection}[1]{%
  \vspace{6pt}%
  {\large\bfseries\MakeUppercase{#1}}%
  \vspace{2pt}%
  \hrule
  \vspace{4pt}%
}

\begin{document}

\begin{center}
{\Large \bfseries YOUR NAME} \\[2pt]
email@example.com $\cdot$ (555) 555-5555 $\cdot$ City, State \\
github.com/username $\cdot$ linkedin.com/in/username $\cdot$ website.com
\end{center}

\resumesection{Summary}
Brief professional summary tailored to the role.

\resumesection{Experience}
\textbf{Job Title} $\cdot$ Company Name \hfill Start -- End \\
\begin{itemize}
\item Achievement bullet point with quantified results.
\item Another achievement relevant to the job description.
\end{itemize}

\resumesection{Projects}
\textbf{Project Name} \hfill github.com/project \\
\begin{itemize}
\item Description of project and technologies used.
\item Impact or outcome of the project.
\end{itemize}

\resumesection{Skills}
\textbf{Languages:} Python, JavaScript, SQL \\
\textbf{Frameworks:} React, Node.js, Django \\
\textbf{Tools:} Git, Docker, AWS

\resumesection{Education}
\textbf{Degree in Field} $\cdot$ Institution \hfill Start -- End

\end{document}
"#
}

fn ats_modern() -> &'static str {
    r#"\documentclass[11pt,letterpaper]{article}

\usepackage[utf8]{inputenc}
\usepackage[T1]{fontenc}
\usepackage{lmodern}
\usepackage[margin=0.75in,top=0.6in,bottom=0.6in]{geometry}
\usepackage{enumitem}
\usepackage{hyperref}
\usepackage{xcolor}
\usepackage{parskip}

\hypersetup{colorlinks=true, urlcolor=blue!70!black, linkcolor=blue!70!black}

\setlist[itemize]{leftmargin=14pt, itemsep=1pt, topsep=2pt, parsep=0pt}

\newcommand{\resumesection}[1]{%
  \vspace{8pt}%
  {\normalsize\bfseries\color{blue!70!black}\MakeUppercase{#1}\color{black}}%
  \vspace{-1pt}%
  \color{blue!70!black}\rule{\textwidth}{1pt}\color{black}%
  \vspace{4pt}%
}

\newcommand{\jobentry}[4]{
  \noindent\textbf{#1} at \textbf{#2} \hfill #3 -- #4 \\[1pt]
}
\newcommand{\projentry}[2]{
  \noindent\textbf{#1} \hfill #2 \\[1pt]
}

\begin{document}

\begin{center}
{\LARGE \bfseries YOUR NAME} \\[4pt]
\small email@example.com $\cdot$ (555) 555-5555 $\cdot$ City, State \\
github.com/username $\cdot$ linkedin.com/in/username
\end{center}
\vspace{2pt}

\resumesection{Professional Summary}
Results-driven professional with expertise in relevant areas. Brief 2-3 line summary tailored to the target role.

\resumesection{Work Experience}
\jobentry{Job Title}{Company Name}{Start Date}{End Date}
\begin{itemize}
\item Key achievement with metrics and impact.
\item Another achievement highlighting relevant skills.
\end{itemize}

\resumesection{Projects}
\projentry{Project Name}{github.com/project}
\begin{itemize}
\item Brief description of the project, technologies, and outcome.
\end{itemize}

\resumesection{Technical Skills}
\textbf{Languages:} Python, JavaScript, TypeScript, SQL \\
\textbf{Frameworks:} React, Node.js, Django, FastAPI \\
\textbf{Tools:} Git, Docker, AWS, CI/CD

\resumesection{Education}
\textbf{Degree in Field}, Institution \hfill Start -- End

\resumesection{Certifications}
\textbf{Certification Name}, Issuing Organization \hfill Date

\end{document}
"#
}

pub fn list_templates() -> Vec<Template> {
    vec![
        Template {
            id: "ats-classic".to_string(),
            name: "ATS Classic".to_string(),
            description: "Clean single-column resume, maximum ATS compatibility. Uses standard fonts and simple formatting.".to_string(),
            source: ats_classic().to_string(),
            built_in: true,
        },
        Template {
            id: "ats-modern".to_string(),
            name: "ATS Modern".to_string(),
            description: "Modern look with accent color rules, still ATS-friendly. Tabular skills section.".to_string(),
            source: ats_modern().to_string(),
            built_in: true,
        },
    ]
}

pub fn get_template_source(id: &str) -> Option<String> {
    list_templates().into_iter().find(|t| t.id == id).map(|t| t.source)
}
