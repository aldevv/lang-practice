import { User } from "@prisma/client";
import jwt from "jsonwebtoken";
import bcrypt from "bcrypt";

export const hashPassword = async (password) => {
  return bcrypt.hash(password, 5);
};

export const comparePasswordHash = async (password, hash) => {
  return bcrypt.compare(password, hash);
};

export const createJWT = (user: User) => {
  const token = jwt.sign(
    {
      id: user.id,
      username: user.username,
    },
    process.env.JWT_SECRET
  );
  return token;
};

export const protect = (req, res, next) => {
  const bearer = req.headers.authorization;
  if (!bearer) {
    res.status(401);
    res.json({ message: "token not provided" });
    return;
  }

  const [, token] = bearer.split(" ");
  console.log(token);
  if (!token) {
    res.status(401);
    res.json({ message: "token not provided" });
    return;
  }
  try {
    const payload = jwt.verify(token, process.env.JWT_SECRET);
    req.user = payload;
    console.log(req.user);
    next();
    return;
  } catch (e) {
    console.error(e);
    res.status(401);
    res.send("Not authorized");
    return;
  }
};
