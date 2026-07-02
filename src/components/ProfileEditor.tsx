import { useState } from "react";
import { useStore } from "../store/useStore";
import { Button } from "./ui/Button";
import { Input } from "./ui/Input";
import { Label } from "./ui/Label";
import { Textarea } from "./ui/Textarea";
import { Card, CardContent, CardHeader, CardTitle } from "./ui/Card";
import { Plus, Trash2, Save, CheckCircle2 } from "lucide-react";
import type { Profile, WorkExperience, Project, SkillGroup, Education, Certification, Award, CustomField } from "../types";
import { newId } from "../types";

export function ProfileEditor() {
  const { profile, updateProfile, saveProfile } = useStore();
  const [showSaved, setShowSaved] = useState(false);

  const handleSave = async () => {
    setShowSaved(true);
    await saveProfile();
    setTimeout(() => setShowSaved(false), 2000);
  };

  const updateBasics = (field: keyof Profile["basics"], value: string) => {
    updateProfile((p) => ({ ...p, basics: { ...p.basics, [field]: value } }));
  };

  return (
    <div className="mx-auto max-w-3xl space-y-6 p-6">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold">Resume Profile</h1>
        <Button onClick={handleSave}>
          {showSaved ? <CheckCircle2 size={16} /> : <Save size={16} />}
          {showSaved ? "Saved!" : "Save Profile"}
        </Button>
      </div>
      <p className="text-sm text-muted-foreground">
        This is your base resume. The AI will tailor it to each job description. Fill in as much as you can.
      </p>

      <Card>
        <CardHeader><CardTitle>Contact & Summary</CardTitle></CardHeader>
        <CardContent className="grid grid-cols-2 gap-3">
          <div className="col-span-2 space-y-1">
            <Label>Full Name</Label>
            <Input value={profile.basics.name} onChange={(e) => updateBasics("name", e.target.value)} placeholder="Jane Doe" />
          </div>
          <div className="space-y-1">
            <Label>Email</Label>
            <Input value={profile.basics.email} onChange={(e) => updateBasics("email", e.target.value)} placeholder="jane@example.com" />
          </div>
          <div className="space-y-1">
            <Label>Phone</Label>
            <Input value={profile.basics.phone} onChange={(e) => updateBasics("phone", e.target.value)} placeholder="(555) 555-5555" />
          </div>
          <div className="space-y-1">
            <Label>Location</Label>
            <Input value={profile.basics.location} onChange={(e) => updateBasics("location", e.target.value)} placeholder="San Francisco, CA" />
          </div>
          <div className="space-y-1">
            <Label>Website</Label>
            <Input value={profile.basics.website} onChange={(e) => updateBasics("website", e.target.value)} placeholder="janedoe.com" />
          </div>
          <div className="space-y-1">
            <Label>GitHub</Label>
            <Input value={profile.basics.github} onChange={(e) => updateBasics("github", e.target.value)} placeholder="github.com/janedoe" />
          </div>
          <div className="space-y-1">
            <Label>LinkedIn</Label>
            <Input value={profile.basics.linkedin} onChange={(e) => updateBasics("linkedin", e.target.value)} placeholder="linkedin.com/in/janedoe" />
          </div>
          <div className="col-span-2 space-y-1">
            <Label>Professional Summary</Label>
            <Textarea
              value={profile.basics.summary}
              onChange={(e) => updateBasics("summary", e.target.value)}
              placeholder="Brief summary of your professional background..."
              rows={3}
            />
          </div>
        </CardContent>
      </Card>

      <WorkSection profile={profile} updateProfile={updateProfile} />
      <ProjectsSection profile={profile} updateProfile={updateProfile} />
      <SkillsSection profile={profile} updateProfile={updateProfile} />
      <EducationSection profile={profile} updateProfile={updateProfile} />
      <CertificationsSection profile={profile} updateProfile={updateProfile} />
      <AwardsSection profile={profile} updateProfile={updateProfile} />
      <CustomSection profile={profile} updateProfile={updateProfile} />
    </div>
  );
}

function WorkSection({ profile, updateProfile }: { profile: Profile; updateProfile: (fn: (p: Profile) => Profile) => void }) {
  const add = () => updateProfile((p) => ({
    ...p,
    work: [...p.work, { id: newId(), company: "", position: "", startDate: "", endDate: "", highlights: [] }],
  }));
  const remove = (id: string) => updateProfile((p) => ({ ...p, work: p.work.filter((w) => w.id !== id) }));
  const update = (id: string, field: keyof WorkExperience, value: any) =>
    updateProfile((p) => ({ ...p, work: p.work.map((w) => (w.id === id ? { ...w, [field]: value } : w)) }));

  return (
    <Card>
      <CardHeader>
        <div className="flex items-center justify-between">
          <CardTitle>Work Experience</CardTitle>
          <Button size="sm" variant="outline" onClick={add}><Plus size={14} /> Add</Button>
        </div>
      </CardHeader>
      <CardContent className="space-y-4">
        {profile.work.length === 0 && <p className="text-sm text-muted-foreground">No work experience added yet.</p>}
        {profile.work.map((w) => (
          <div key={w.id} className="space-y-2 rounded-md border border-border p-3">
            <div className="flex justify-between">
              <div className="grid flex-1 grid-cols-2 gap-2">
                <Input value={w.position} onChange={(e) => update(w.id, "position", e.target.value)} placeholder="Position" />
                <Input value={w.company} onChange={(e) => update(w.id, "company", e.target.value)} placeholder="Company" />
                <Input value={w.startDate} onChange={(e) => update(w.id, "startDate", e.target.value)} placeholder="Start (Jan 2023)" />
                <Input value={w.endDate} onChange={(e) => update(w.id, "endDate", e.target.value)} placeholder="End (Present)" />
              </div>
              <Button size="sm" variant="ghost" onClick={() => remove(w.id)}><Trash2 size={14} /></Button>
            </div>
            <div className="space-y-1">
              <Label>Highlights (one per line)</Label>
              <Textarea
                value={w.highlights.join("\n")}
                onChange={(e) => update(w.id, "highlights", e.target.value.split("\n").filter((l) => l.trim()))}
                placeholder={"Built X that improved Y by Z%\nLed team of N to deliver..."}
                rows={3}
              />
            </div>
          </div>
        ))}
      </CardContent>
    </Card>
  );
}

function ProjectsSection({ profile, updateProfile }: { profile: Profile; updateProfile: (fn: (p: Profile) => Profile) => void }) {
  const add = () => updateProfile((p) => ({
    ...p,
    projects: [...p.projects, { id: newId(), name: "", description: "", tech: [], url: "", highlights: [] }],
  }));
  const remove = (id: string) => updateProfile((p) => ({ ...p, projects: p.projects.filter((x) => x.id !== id) }));
  const update = (id: string, field: keyof Project, value: any) =>
    updateProfile((p) => ({ ...p, projects: p.projects.map((x) => (x.id === id ? { ...x, [field]: value } : x)) }));

  return (
    <Card>
      <CardHeader>
        <div className="flex items-center justify-between">
          <CardTitle>Projects</CardTitle>
          <Button size="sm" variant="outline" onClick={add}><Plus size={14} /> Add</Button>
        </div>
      </CardHeader>
      <CardContent className="space-y-4">
        {profile.projects.length === 0 && <p className="text-sm text-muted-foreground">No projects added yet.</p>}
        {profile.projects.map((p) => (
          <div key={p.id} className="space-y-2 rounded-md border border-border p-3">
            <div className="flex justify-between">
              <div className="grid flex-1 grid-cols-2 gap-2">
                <Input value={p.name} onChange={(e) => update(p.id, "name", e.target.value)} placeholder="Project Name" />
                <Input value={p.url} onChange={(e) => update(p.id, "url", e.target.value)} placeholder="github.com/..." />
              </div>
              <Button size="sm" variant="ghost" onClick={() => remove(p.id)}><Trash2 size={14} /></Button>
            </div>
            <Input value={p.tech.join(", ")} onChange={(e) => update(p.id, "tech", e.target.value.split(",").map((t) => t.trim()).filter(Boolean))} placeholder="React, Node.js, PostgreSQL" />
            <Textarea value={p.description} onChange={(e) => update(p.id, "description", e.target.value)} placeholder="Project description" rows={2} />
            <div className="space-y-1">
              <Label>Highlights (one per line)</Label>
              <Textarea
                value={p.highlights.join("\n")}
                onChange={(e) => update(p.id, "highlights", e.target.value.split("\n").filter((l) => l.trim()))}
                placeholder={"Achieved X...\nBuilt with Y..."}
                rows={2}
              />
            </div>
          </div>
        ))}
      </CardContent>
    </Card>
  );
}

function SkillsSection({ profile, updateProfile }: { profile: Profile; updateProfile: (fn: (p: Profile) => Profile) => void }) {
  const add = () => updateProfile((p) => ({
    ...p,
    skills: [...p.skills, { id: newId(), category: "", items: [] }],
  }));
  const remove = (id: string) => updateProfile((p) => ({ ...p, skills: p.skills.filter((s) => s.id !== id) }));
  const update = (id: string, field: keyof SkillGroup, value: any) =>
    updateProfile((p) => ({ ...p, skills: p.skills.map((s) => (s.id === id ? { ...s, [field]: value } : s)) }));

  return (
    <Card>
      <CardHeader>
        <div className="flex items-center justify-between">
          <CardTitle>Skills</CardTitle>
          <Button size="sm" variant="outline" onClick={add}><Plus size={14} /> Add Group</Button>
        </div>
      </CardHeader>
      <CardContent className="space-y-3">
        {profile.skills.length === 0 && <p className="text-sm text-muted-foreground">No skills added yet.</p>}
        {profile.skills.map((s) => (
          <div key={s.id} className="flex items-center gap-2">
            <Input value={s.category} onChange={(e) => update(s.id, "category", e.target.value)} placeholder="Category (Languages)" className="w-40" />
            <Input value={s.items.join(", ")} onChange={(e) => update(s.id, "items", e.target.value.split(",").map((t) => t.trim()).filter(Boolean))} placeholder="Python, JavaScript, SQL" className="flex-1" />
            <Button size="sm" variant="ghost" onClick={() => remove(s.id)}><Trash2 size={14} /></Button>
          </div>
        ))}
      </CardContent>
    </Card>
  );
}

function EducationSection({ profile, updateProfile }: { profile: Profile; updateProfile: (fn: (p: Profile) => Profile) => void }) {
  const add = () => updateProfile((p) => ({
    ...p,
    education: [...p.education, { id: newId(), institution: "", degree: "", field: "", startDate: "", endDate: "", gpa: "", highlights: [] }],
  }));
  const remove = (id: string) => updateProfile((p) => ({ ...p, education: p.education.filter((e) => e.id !== id) }));
  const update = (id: string, field: keyof Education, value: any) =>
    updateProfile((p) => ({ ...p, education: p.education.map((e) => (e.id === id ? { ...e, [field]: value } : e)) }));

  return (
    <Card>
      <CardHeader>
        <div className="flex items-center justify-between">
          <CardTitle>Education</CardTitle>
          <Button size="sm" variant="outline" onClick={add}><Plus size={14} /> Add</Button>
        </div>
      </CardHeader>
      <CardContent className="space-y-3">
        {profile.education.length === 0 && <p className="text-sm text-muted-foreground">No education added yet.</p>}
        {profile.education.map((e) => (
          <div key={e.id} className="space-y-2 rounded-md border border-border p-3">
            <div className="flex justify-between">
              <div className="grid flex-1 grid-cols-2 gap-2">
                <Input value={e.institution} onChange={(ev) => update(e.id, "institution", ev.target.value)} placeholder="University" />
                <Input value={e.degree} onChange={(ev) => update(e.id, "degree", ev.target.value)} placeholder="B.S." />
                <Input value={e.field} onChange={(ev) => update(e.id, "field", ev.target.value)} placeholder="Computer Science" />
                <Input value={e.gpa} onChange={(ev) => update(e.id, "gpa", ev.target.value)} placeholder="GPA: 3.8" />
                <Input value={e.startDate} onChange={(ev) => update(e.id, "startDate", ev.target.value)} placeholder="Start" />
                <Input value={e.endDate} onChange={(ev) => update(e.id, "endDate", ev.target.value)} placeholder="End" />
              </div>
              <Button size="sm" variant="ghost" onClick={() => remove(e.id)}><Trash2 size={14} /></Button>
            </div>
          </div>
        ))}
      </CardContent>
    </Card>
  );
}

function CertificationsSection({ profile, updateProfile }: { profile: Profile; updateProfile: (fn: (p: Profile) => Profile) => void }) {
  const add = () => updateProfile((p) => ({
    ...p,
    certifications: [...p.certifications, { id: newId(), name: "", issuer: "", date: "" }],
  }));
  const remove = (id: string) => updateProfile((p) => ({ ...p, certifications: p.certifications.filter((c) => c.id !== id) }));
  const update = (id: string, field: keyof Certification, value: string) =>
    updateProfile((p) => ({ ...p, certifications: p.certifications.map((c) => (c.id === id ? { ...c, [field]: value } : c)) }));

  return (
    <Card>
      <CardHeader>
        <div className="flex items-center justify-between">
          <CardTitle>Certifications</CardTitle>
          <Button size="sm" variant="outline" onClick={add}><Plus size={14} /> Add</Button>
        </div>
      </CardHeader>
      <CardContent className="space-y-3">
        {profile.certifications.length === 0 && <p className="text-sm text-muted-foreground">No certifications added yet.</p>}
        {profile.certifications.map((c) => (
          <div key={c.id} className="flex items-center gap-2">
            <Input value={c.name} onChange={(e) => update(c.id, "name", e.target.value)} placeholder="Certification Name" className="flex-1" />
            <Input value={c.issuer} onChange={(e) => update(c.id, "issuer", e.target.value)} placeholder="Issuer" className="w-40" />
            <Input value={c.date} onChange={(e) => update(c.id, "date", e.target.value)} placeholder="Date" className="w-28" />
            <Button size="sm" variant="ghost" onClick={() => remove(c.id)}><Trash2 size={14} /></Button>
          </div>
        ))}
      </CardContent>
    </Card>
  );
}

function AwardsSection({ profile, updateProfile }: { profile: Profile; updateProfile: (fn: (p: Profile) => Profile) => void }) {
  const add = () => updateProfile((p) => ({
    ...p,
    awards: [...p.awards, { id: newId(), title: "", date: "", awarder: "" }],
  }));
  const remove = (id: string) => updateProfile((p) => ({ ...p, awards: p.awards.filter((a) => a.id !== id) }));
  const update = (id: string, field: keyof Award, value: string) =>
    updateProfile((p) => ({ ...p, awards: p.awards.map((a) => (a.id === id ? { ...a, [field]: value } : a)) }));

  return (
    <Card>
      <CardHeader>
        <div className="flex items-center justify-between">
          <CardTitle>Awards</CardTitle>
          <Button size="sm" variant="outline" onClick={add}><Plus size={14} /> Add</Button>
        </div>
      </CardHeader>
      <CardContent className="space-y-3">
        {profile.awards.length === 0 && <p className="text-sm text-muted-foreground">No awards added yet.</p>}
        {profile.awards.map((a) => (
          <div key={a.id} className="flex items-center gap-2">
            <Input value={a.title} onChange={(e) => update(a.id, "title", e.target.value)} placeholder="Award Title" className="flex-1" />
            <Input value={a.awarder} onChange={(e) => update(a.id, "awarder", e.target.value)} placeholder="Awarder" className="w-40" />
            <Input value={a.date} onChange={(e) => update(a.id, "date", e.target.value)} placeholder="Date" className="w-28" />
            <Button size="sm" variant="ghost" onClick={() => remove(a.id)}><Trash2 size={14} /></Button>
          </div>
        ))}
      </CardContent>
    </Card>
  );
}

function CustomSection({ profile, updateProfile }: { profile: Profile; updateProfile: (fn: (p: Profile) => Profile) => void }) {
  const add = () => updateProfile((p) => ({
    ...p,
    custom: [...p.custom, { id: newId(), key: "", value: "" }],
  }));
  const remove = (id: string) => updateProfile((p) => ({ ...p, custom: p.custom.filter((c) => c.id !== id) }));
  const update = (id: string, field: keyof CustomField, value: string) =>
    updateProfile((p) => ({ ...p, custom: p.custom.map((c) => (c.id === id ? { ...c, [field]: value } : c)) }));

  return (
    <Card>
      <CardHeader>
        <div className="flex items-center justify-between">
          <CardTitle>Custom Fields</CardTitle>
          <Button size="sm" variant="outline" onClick={add}><Plus size={14} /> Add</Button>
        </div>
      </CardHeader>
      <CardContent className="space-y-3">
        {profile.custom.length === 0 && <p className="text-sm text-muted-foreground">Add any custom key-value fields.</p>}
        {profile.custom.map((c) => (
          <div key={c.id} className="flex items-center gap-2">
            <Input value={c.key} onChange={(e) => update(c.id, "key", e.target.value)} placeholder="Label" className="w-40" />
            <Input value={c.value} onChange={(e) => update(c.id, "value", e.target.value)} placeholder="Value" className="flex-1" />
            <Button size="sm" variant="ghost" onClick={() => remove(c.id)}><Trash2 size={14} /></Button>
          </div>
        ))}
      </CardContent>
    </Card>
  );
}
