import pygame
import sys
import random

pygame.display.set_caption("Snake")


class Block:
    def __init__(self, x_pos, y_pos):
        self.x = x_pos
        self.y = y_pos


class Food:
    def __init__(self):
        x = random.randint(0, NB_COL - 1)
        y = random.randint(0, NB_ROW - 1)
        self.block = Block(x, y)

    def draw_food(self):
        nourriture = pygame.Rect(self.block.x * CELL_SIZE, self.block.y * CELL_SIZE, CELL_SIZE, CELL_SIZE)
        pygame.draw.rect(screen, pygame.Color("red"), nourriture)


class Snake:
    def __init__(self):
        self.body = [Block(2, 6), Block(3, 6), Block(4, 6)]
        self.direction = "RIGHT"

    def draw_snake(self):
        for block in self.body:
            x_coord = block.x * CELL_SIZE
            y_coord = block.y * CELL_SIZE
            block_rect = pygame.Rect(x_coord, y_coord, CELL_SIZE, CELL_SIZE)
            pygame.draw.rect(screen, (0, 255, 0), block_rect)

    def move_snake(self):
        snake_block_count = len(self.body)
        old_head = self.body[snake_block_count - 1]

        if self.direction == "RIGHT":
            new_head = Block(old_head.x + 1, old_head.y)

        elif self.direction == "LEFT":
            new_head = Block(old_head.x - 1, old_head.y)

        elif self.direction == "DOWN":
            new_head = Block(old_head.x, old_head.y + 1)

        else:
            new_head = Block(old_head.x, old_head.y - 1)

        self.body.append(new_head)

    def reset_snake(self):
        self.body = [Block(2, 6), Block(3, 6), Block(4, 6)]
        self.direction = "RIGHT"


class Game:
    def __init__(self):
        self.snake = Snake()
        self.food = Food()
        self.generate_food()

    def update(self):
        self.snake.move_snake()
        self.check_head_on_food()
        self.game_over()

    def draw_game_element(self):
        self.food.draw_food()
        self.snake.draw_snake()

    def check_head_on_food(self):
        global score
        snake_length = len(self.snake.body)
        snake_head_block = self.snake.body[snake_length - 1]
        food_block = self.food.block
        if snake_head_block.x == food_block.x and snake_head_block.y == food_block.y:
            self.generate_food()
            score += 1
        else:
            self.snake.body.pop(0)

    def generate_food(self):
        should_generate_food = True
        while should_generate_food:
            count = 0
            for block in self.snake.body:
                if block.x == self.food.block.x and block.y == self.food.block.y:
                    count += 1
            if count == 0:
                should_generate_food = False
            else:
                self.food = Food()

    def game_over(self):
        snake_length = len(self.snake.body)
        snake_head = self.snake.body[snake_length - 1]
        if snake_head.x not in range(0, NB_COL) or snake_head.y not in range(0, NB_ROW):
            self.snake.reset_snake()
        for block in self.snake.body[0:snake_length - 1]:
            if block.x == snake_head.x and block.y == snake_head.y:
                self.snake.reset_snake()

    def win(self):
        snake_length = len(self.snake.body)
        if snake_length == (NB_COL - 1) * (NB_ROW - 1):
            print("Win :)")
    def score(self):
        display_score = font.render("Score : " + str(score), True, (0, 0, 0), (255, 255, 255))


pygame.init()

NB_COL = 10
NB_ROW = 15
CELL_SIZE = 40
score = 0

screen = pygame.display.set_mode(size=(NB_COL * CELL_SIZE, NB_ROW * CELL_SIZE))
timer = pygame.time.Clock()
font = pygame.font.Font("Consolas.ttf", 20)

game_on = True
game = Game()

SCREEN_UPDATE = pygame.USEREVENT
pygame.time.set_timer(SCREEN_UPDATE, 200)

while game_on:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            pygame.quit()
            sys.exit()
        if event.type == SCREEN_UPDATE:
            game.update()

        if event.type == pygame.KEYDOWN:
            if event.key == pygame.K_UP and game.snake.direction != "DOWN":
                game.snake.direction = "UP"
            if event.key == pygame.K_DOWN and game.snake.direction != "UP":
                game.snake.direction = "DOWN"
            if event.key == pygame.K_LEFT and game.snake.direction != "RIGHT":
                game.snake.direction = "LEFT"
            if event.key == pygame.K_RIGHT and game.snake.direction != "LEFT":
                game.snake.direction = "RIGHT"
    screen.fill((255, 255, 255))
    game.draw_game_element()
    pygame.display.update()
    timer.tick(60)

pygame.quit()
