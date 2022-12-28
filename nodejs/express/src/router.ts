import { Router } from "express";
import { body } from "express-validator";
import {
  createProduct,
  deleteProduct,
  getOneProduct,
  getProducts,
  updateProduct,
} from "./handlers/product";
import { getOneUpdate, getUpdates } from "./handlers/update";
import { handleInputErrors } from "./modules/middleware";

const router = Router();
/**
 * Product
 */
router.get("/product", getProducts);

router.get("/product/:id", getOneProduct);

router.post(
  "/product",
  body("name").isString(),
  handleInputErrors,
  createProduct
);

router.put("/product/:id", body("name"), handleInputErrors, updateProduct);

router.delete("/product/:id", deleteProduct);

/**
 * Update
 */

router.get("/update", getUpdates) => { });

router.get("/update/:id", getOneUpdate) => { });

router.post("/update",
  body('title').optional(),
  body('body').optional(),
  body('status').optional(),
  body('version').optional()
);

router.put("/update/:id", (req, res) => { });

router.delete("/update/:id", (req, res) => { });

/**
 * UpdatePoint
 */

router.get("/updatepoint", (req, res) => { });

router.get("/updatepoint/:id", (req, res) => { });

router.post("/updatepoint", (req, res) => { });

router.put("/updatepoint/:id", (req, res) => { });

router.delete("/updatepoint/:id", (req, res) => { });

export default router;
