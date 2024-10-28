
// import { PDFReader } from "llamaindex";
// import fs from "fs";


// export async function getDocuments() {
//   const DATA_DIR = "../rust-pdf-extract-chunking/docs/pdf2.0.pdf";
//   const reader = new PDFReader();
//   const output = await reader.loadData(DATA_DIR);
//   // Load PDFs using LlamaParseReader

//   console.log(`🚀 ~ output:`, output);
//   // write output to a txt file
//   fs.writeFileSync("output.txt", output[0].text);
// }

// await getDocuments()


// import fs from "node:fs/promises";

// import {
//   Settings,
//   Ollama,
//   HuggingFaceEmbedding,
//   Document,
//   VectorStoreIndex,
// } from "llamaindex";

// async function main() {
//   Settings.llm = new Ollama({
//     model: "llama3.1",
//   });
//   Settings.embedModel = new HuggingFaceEmbedding({
//     modelType: "BAAI/bge-small-en-v1.5",
//     quantized: false,
//   });
  
//   // Load essay from abramov.txt in Node
//   const path = "node_modules/llamaindex/examples/abramov.txt";

//   const essay = await fs.readFile(path, "utf-8");

//   // Create Document object with essay
//   const document = new Document({ text: essay, id_: path });

//   // Split text and create embeddings. Store them in a VectorStoreIndex
//   const index = await VectorStoreIndex.fromDocuments([document]);

//   // Query the index
//   const queryEngine = index.asQueryEngine();

//   const response = await queryEngine.query({
//     query: "What did the author do in college?",
//   });

//   // Output response
//   console.log(response.toString());
// }

// main().catch(console.error);


import { LlamaParseReader, VectorStoreIndex } from "llamaindex";
import 'dotenv/config'


async function main() {
  // Load PDF using LlamaParse
    const DATA_DIR = "../rust-pdf-extract-chunking/docs/pdf2.0.pdf";

  const reader = new LlamaParseReader({ resultType: "markdown" });
  const documents = await reader.loadData(DATA_DIR);
  console.log(`🚀 ~ documents:`, documents);

  // // Split text and create embeddings. Store them in a VectorStoreIndex
  // const index = await VectorStoreIndex.fromDocuments(documents);

  // // Query the index
  // const queryEngine = index.asQueryEngine();
  // const response = await queryEngine.query({
  //   query: "What is the license grant in the TOS?",
  // });

  // Output response
  // console.log(response.toString());
}

main().catch(console.error);
