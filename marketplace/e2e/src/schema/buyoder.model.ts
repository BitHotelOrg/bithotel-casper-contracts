import { Schema, model } from "mongoose";

interface IBuyOrder {
  creator: string;
  asset: Schema.Types.ObjectId;
  owner: string;
  payToken?: string;
  price: string;
  startTime: number;
  additionalRecipient?: string;
  status: "pending" | "succeed" | "canceled";
}

const buyOrderSchema = new Schema<IBuyOrder>({
  creator: {
    type: String,
    required: true,
  },
  asset: {
    type: Schema.Types.ObjectId,
    required: true,
  },
  owner: {
    type: String,
    required: true,
  },
  payToken: {
    type: String,
  },
  price: {
    type: String,
    required: true,
  },
  startTime: {
    type: Number,
    required: true,
  },
  additionalRecipient: {
    type: String,
  },
  status: {
    type: String,
    enum: ["pending", "succed", "canceld"],
    required: true,
  },
});

const BuyOrder = model<IBuyOrder>("BuyOrder", buyOrderSchema);

export default BuyOrder;
