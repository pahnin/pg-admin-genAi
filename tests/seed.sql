-- Users
INSERT INTO users (email, password_hash, full_name)
VALUES
('alice@example.com', 'hashed_pw1', 'Alice Wonderland'),
('bob@example.com', 'hashed_pw2', 'Bob Builder'),
('carol@example.com', 'hashed_pw3', 'Carol Singer');

-- Roles
INSERT INTO roles (name) VALUES ('admin'), ('customer'), ('seller');

-- Permissions
INSERT INTO permissions (name)
VALUES ('manage_users'), ('manage_orders'), ('manage_books'), ('view_reports');

-- User Roles
INSERT INTO user_roles (user_id, role_id)
VALUES (1, 1), (2, 2), (3, 2);

-- Role Permissions
INSERT INTO role_permissions (role_id, permission_id)
VALUES
(1, 1), (1, 2), (1, 3), (1, 4),
(2, 2), (2, 3),
(3, 3);

-- Addresses
INSERT INTO addresses (user_id, street, city, state, zip, country)
VALUES
(1, '123 Elm Street', 'Wonderland', 'MagicState', '11111', 'Fictionland'),
(2, '456 Builder Rd', 'ConstructCity', 'Workland', '22222', 'Fixland'),
(3, '789 Melody Ln', 'SingTown', 'Harmonystate', '33333', 'Songland');

-- Authors
INSERT INTO authors (name, bio)
VALUES
('J.K. Rowling', 'British author of Harry Potter.'),
('George R.R. Martin', 'American novelist, Game of Thrones.'),
('J.R.R. Tolkien', 'English writer, The Lord of the Rings.');

-- Publishers
INSERT INTO publishers (name, country)
VALUES
('Bloomsbury', 'UK'),
('Bantam Books', 'USA'),
('Allen & Unwin', 'UK');

-- Books
INSERT INTO books (title, isbn, publisher_id, published_date, price)
VALUES
('Harry Potter and the Philosopher''s Stone', '9780747532699', 1, '1997-06-26', 19.99),
('A Game of Thrones', '9780553103540', 2, '1996-08-06', 24.99),
('The Fellowship of the Ring', '9780048231887', 3, '1954-07-29', 29.99);

-- Categories
INSERT INTO categories (name)
VALUES ('Fantasy'), ('Adventure'), ('Drama');

-- Book Categories
INSERT INTO book_categories (book_id, category_id)
VALUES (1, 1), (2, 1), (2, 3), (3, 1), (3, 2);

-- Book Authors
INSERT INTO book_authors (book_id, author_id)
VALUES (1, 1), (2, 2), (3, 3);

-- Inventory
INSERT INTO inventory (book_id, stock)
VALUES (1, 50), (2, 30), (3, 20);

-- Orders
INSERT INTO orders (user_id, status, total_amount)
VALUES (1, 'paid', 44.98), (2, 'shipped', 29.99);

-- Order Items
INSERT INTO order_items (order_id, book_id, quantity, price)
VALUES (1, 1, 1, 19.99), (1, 2, 1, 24.99), (2, 3, 1, 29.99);

-- Payments
INSERT INTO payments (order_id, amount, method, status, paid_at)
VALUES (1, 44.98, 'credit_card', 'completed', now()),
       (2, 29.99, 'paypal', 'completed', now());

-- Shipments
INSERT INTO shipments (order_id, address_id, status, shipped_at)
VALUES (2, 2, 'in_transit', now());

-- Shipment Items
INSERT INTO shipment_items (shipment_id, order_item_id)
VALUES (1, 3);

-- Reviews
INSERT INTO reviews (user_id, book_id, rating, comment)
VALUES (1, 1, 5, 'Magical!'), (2, 2, 4, 'Epic read!');

-- Coupons
INSERT INTO coupons (code, discount_percent, valid_from, valid_to)
VALUES ('SUMMER10', 10, '2025-06-01', '2025-08-31');

-- Wishlist
INSERT INTO wishlist (user_id, book_id)
VALUES (1, 2), (2, 1);

-- Carts
INSERT INTO carts (user_id) VALUES (3);

-- Cart Items
INSERT INTO cart_items (cart_id, book_id, quantity)
VALUES (1, 3, 2);

-- Events
INSERT INTO events (user_id, event_type, payload)
VALUES (1, 'login', '{"ip": "127.0.0.1"}');

-- Audit Logs
INSERT INTO audit_logs (user_id, action, table_name, record_id)
VALUES (1, 'INSERT', 'books', 1);


-- === Users ===
INSERT INTO users (email, password_hash, full_name) VALUES
('alice@example.com', 'hash1', 'Alice Wonderland'),
('bob@example.com', 'hash2', 'Bob Builder'),
('carol@example.com', 'hash3', 'Carol Singer'),
('dave@example.com', 'hash4', 'Dave Reader'),
('eve@example.com', 'hash5', 'Eve Coder'),
('frank@example.com', 'hash6', 'Frank Writer'),
('grace@example.com', 'hash7', 'Grace Poet'),
('heidi@example.com', 'hash8', 'Heidi Scholar');

-- === Roles ===
INSERT INTO roles (name) VALUES
('admin'), ('customer'), ('seller');

-- === Permissions ===
INSERT INTO permissions (name) VALUES
('manage_users'), ('manage_orders'), ('manage_books'), ('view_reports');

-- === User Roles ===
INSERT INTO user_roles (user_id, role_id) VALUES
(1,1), (2,2), (3,2), (4,2), (5,3), (6,2), (7,2), (8,3);

-- === Role Permissions ===
INSERT INTO role_permissions (role_id, permission_id) VALUES
(1,1), (1,2), (1,3), (1,4),
(2,2), (2,3),
(3,3);

-- === Addresses ===
INSERT INTO addresses (user_id, street, city, state, zip, country) VALUES
(1,'123 Elm St','Wonderland','Magic','11111','Fictionland'),
(2,'456 Builder Rd','ConstructCity','Workland','22222','Fixland'),
(3,'789 Melody Ln','SingTown','Harmonystate','33333','Songland'),
(4,'111 River St','Readville','Bookland','44444','Novelstan'),
(5,'222 Code Ave','TechCity','Devstate','55555','Codeland'),
(6,'333 Writer Way','StoryTown','Literaria','66666','Writania'),
(7,'444 Poem St','VerseVille','Poetry','77777','Rhymland'),
(8,'555 Scholar Dr','Academia','Learnland','88888','Thinkland');

-- === Authors ===
INSERT INTO authors (name,bio) VALUES
('J.K. Rowling','Harry Potter'),
('George R.R. Martin','Game of Thrones'),
('J.R.R. Tolkien','Lord of the Rings'),
('Isaac Asimov','Sci-Fi author'),
('Agatha Christie','Mystery novelist'),
('Stephen King','Horror master'),
('Neil Gaiman','Fantasy & Comics'),
('Brandon Sanderson','Epic fantasy writer');

-- === Publishers ===
INSERT INTO publishers (name,country) VALUES
('Bloomsbury','UK'),
('Bantam Books','USA'),
('Allen & Unwin','UK'),
('HarperCollins','USA'),
('Penguin Random House','UK');

-- === Books ===
INSERT INTO books (title,isbn,publisher_id,published_date,price) VALUES
('Harry Potter 1','9780747532699',1,'1997-06-26',19.99),
('Game of Thrones','9780553103540',2,'1996-08-06',24.99),
('Fellowship of the Ring','9780048231887',3,'1954-07-29',29.99),
('Foundation','9780553293357',2,'1951-06-01',15.99),
('Murder on the Orient Express','9780062073501',4,'1934-01-01',12.99),
('The Shining','9780307743657',5,'1977-01-28',18.50),
('American Gods','9780380789030',4,'2001-06-19',22.00),
('Mistborn','9780765311788',5,'2006-07-17',21.50);

-- === Categories ===
INSERT INTO categories (name) VALUES
('Fantasy'), ('Adventure'), ('Drama'),
('Mystery'), ('Horror'), ('Sci-Fi'), ('Romance');

-- === Book Categories ===
INSERT INTO book_categories (book_id, category_id) VALUES
(1,1), (1,2), (2,1), (2,3), (3,1), (3,2),
(4,6), (5,4), (6,5), (7,1), (7,3), (8,1), (8,2);

-- === Book Authors ===
INSERT INTO book_authors (book_id,author_id) VALUES
(1,1), (2,2), (3,3), (4,4), (5,5), (6,6), (7,7), (8,8);

-- === Inventory ===
INSERT INTO inventory (book_id,stock) VALUES
(1,50),(2,30),(3,20),(4,40),(5,25),(6,15),(7,35),(8,45);

-- === Orders ===
INSERT INTO orders (user_id,status,total_amount) VALUES
(1,'paid',44.98),
(2,'shipped',29.99),
(3,'pending',15.99),
(4,'paid',52.49),
(5,'cancelled',0.00);

-- === Order Items ===
INSERT INTO order_items (order_id,book_id,quantity,price) VALUES
(1,1,1,19.99),(1,2,1,24.99),
(2,3,1,29.99),
(3,4,1,15.99),
(4,5,1,12.99),(4,6,1,18.50),(4,7,1,21.00),
(5,8,1,21.50);

-- === Payments ===
INSERT INTO payments (order_id,amount,method,status,paid_at) VALUES
(1,44.98,'credit_card','completed',now()),
(2,29.99,'paypal','completed',now()),
(3,15.99,'credit_card','failed',now()),
(4,52.49,'debit_card','completed',now());

-- === Shipments ===
INSERT INTO shipments (order_id,address_id,status,shipped_at,delivered_at) VALUES
(2,2,'in_transit',now(),NULL),
(4,4,'delivered',now(),now());

-- === Shipment Items ===
INSERT INTO shipment_items (shipment_id,order_item_id) VALUES
(1,3),(2,5),(2,6),(2,7);

-- === Reviews ===
INSERT INTO reviews (user_id,book_id,rating,comment) VALUES
(1,1,5,'Magical!'),
(2,2,4,'Epic read!'),
(3,3,5,'Classic fantasy'),
(4,4,3,'Interesting sci-fi'),
(5,5,4,'Loved the twist'),
(6,6,2,'Too scary'),
(7,7,5,'Masterpiece'),
(8,8,4,'Great worldbuilding');

-- === Coupons ===
INSERT INTO coupons (code,discount_percent,valid_from,valid_to) VALUES
('SUMMER10',10,'2025-06-01','2025-08-31'),
('NEWUSER20',20,'2025-01-01','2025-12-31'),
('FALL5',5,'2025-09-01','2025-11-30');

-- === Wishlist ===
INSERT INTO wishlist (user_id,book_id) VALUES
(1,2),(2,1),(3,3),(4,4),(5,5),(6,6),(7,7),(8,8);

-- === Carts ===
INSERT INTO carts (user_id) VALUES
(1),(2),(3),(4),(5);

-- === Cart Items ===
INSERT INTO cart_items (cart_id,book_id,quantity) VALUES
(1,1,2),(1,3,1),
(2,2,1),
(3,4,2),
(4,5,1),(4,6,1),
(5,7,3);

-- === Events ===
INSERT INTO events (user_id,event_type,payload) VALUES
(1,'login','{"ip":"127.0.0.1"}'),
(2,'order_created','{"order_id":2}'),
(3,'review_added','{"book_id":3}'),
(4,'coupon_used','{"code":"SUMMER10"}');

-- === Audit Logs ===
INSERT INTO audit_logs (user_id,action,table_name,record_id) VALUES
(1,'INSERT','books',1),
(2,'UPDATE','orders',2),
(3,'DELETE','wishlist',3),
(4,'INSERT','reviews',4);

