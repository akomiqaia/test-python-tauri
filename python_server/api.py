from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
import uvicorn
from sentence_transformers import SentenceTransformer

model = SentenceTransformer("all-MiniLM-L6-v2")

app = FastAPI()

origins = [
    "*"
]

app.add_middleware(
    CORSMiddleware,
    allow_origins=origins,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

class Item(BaseModel):
    name: str
    price: float

@app.get("/")
def read_root():
    return {"Hello": "World"}

@app.get("/sim")
def read_item():
    # Two lists of sentences
    sentences1 = [
        "The new movie is awesome",
        "The cat sits outside",
        "A man is playing guitar",
    ]

    sentences2 = [
        "The dog plays in the garden",
        "The new movie is so great",
        "A woman watches TV",
    ]

    # Compute embeddings for both lists
    embeddings1 = model.encode(sentences1)
    embeddings2 = model.encode(sentences2)

    # Compute cosine similarities
    similarities = model.similarity(embeddings1, embeddings2)
    output = []
    # Output the pairs with their score
    for idx_i, sentence1 in enumerate(sentences1):
        print(sentence1)
        for idx_j, sentence2 in enumerate(sentences2):
            print(f" - {sentence2: <30}: {similarities[idx_i][idx_j]:.4f}")
            # add the similarity score to the output
            output.append({"sentence1": sentence1, "sentence2": sentence2, "similarity": f" - {sentence2: <30}: {similarities[idx_i][idx_j]:.4f}"})
    return {"item_id": output}
if __name__ == "__main__":
    uvicorn.run(app, host="127.0.0.1", port=8000)
