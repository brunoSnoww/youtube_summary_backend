# youtube_summary_backend (WIP)
Repo for testing a program that downloads audio from Youtube and summarises it with AI

Terminal 1
```bash
export OPENAI_API_KEY=your-key
cargo run
```

Terminal 2
```bash
curl -X POST -H "Content-Type: application/json" \
  -d '{"youtube_url": "https://www.youtube.com/watch?v=fMCN-b0ic_k", "language": "en"}' \
  http://localhost:3000/summarize


{"summary":"The transcript provided is from the song \"Never Gonna Give You Up\" by Rick Astley. The lyrics describe a person who is expressing their desire to end a relationship, indicating that they want to make the other person cry and say goodbye. The tone of the lyrics suggests a sense of sadness or resignation in the decision to part ways with the other person."}%
```
