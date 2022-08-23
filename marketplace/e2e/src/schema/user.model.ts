import { Schema, model } from "mongoose";

interface IUser {
  name: string;
  email?: string;
  accountHash: string;
  nonce: string;
  // role:"user"|"minter"|"admin"
}

const userSchema = new Schema<IUser>({
  name: {
    type: String,
    required: true,
  },
  email: {
    type: String,
    unique: true,
    dropDups: true,
  },
  accountHash: {
    type: String,
    required: true,
    unique: true,
    dropDups: true,
  },
});

const User = model<IUser>("User", userSchema);

export default User;
