import { createJWT, comparePasswordHash, hashPassword } from "../modules/auth";
import prisma from "../modules/db";

export const createUser = async (req, res) => {
  console.log(req.body);
  const user = await prisma.user.create({
    data: {
      username: req.body.username,
      password: await hashPassword(req.body.password),
    },
  });

  const token = createJWT(user);
  res.json({ token });
};

export const signin = async (req, res) => {
  const user = await prisma.user.findUnique({
    where: {
      username: req.body.username,
    },
  });
  const isValid: boolean = await comparePasswordHash(
    req.body.password,
    user.password
  );

  if (!isValid) {
    res.status(401);
    res.json({ message: "who are you trying to hack bro" });
  }
  const token = createJWT(user);
  res.json({ token });
};
