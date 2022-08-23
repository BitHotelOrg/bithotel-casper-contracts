import { Schema, model } from "mongoose";

interface ICollection {
  contractPackageHash: string;
  contractHash: string;
  slug: string;
  name: string;
  symbol: string;
  description?: string;
  verified: boolean;
  image?: string;
  twitter?: string;
  discord?: string;
  website?: string;
}

const collectionSchema = new Schema<ICollection>({
  contractPackageHash: {
    type: String,
    required: true,
    unique: true,
  },
  contractHash: {
    type: String,
    required: true,
    unique: true,
    dropDups: true,
  },
  slug: { type: String, required: true, unique: true, dropDups: true },
  symbol: { type: String, required: true },
  name: { type: String, required: true },
  description: { type: String },
  verified: { type: Boolean, required: true },
  image: { type: String },
  twitter: { type: String },
  discord: { type: String },
  website: { type: String },
});

const Collection = model<ICollection>("Collection", collectionSchema);

export default Collection;
