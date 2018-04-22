package com.grouter.main;

import com.badlogic.gdx.Gdx;
import com.badlogic.gdx.graphics.Color;
import com.badlogic.gdx.graphics.glutils.ShapeRenderer;
import com.badlogic.gdx.math.Vector2;

public class BouncingBall {

    private float radius;
    private Vector2 position;
    private Vector2 velocity;

    public BouncingBall(float initial_x, float initial_y, float speed, float radius){
        this.radius = radius;
        position = new Vector2(initial_x, initial_y);
        velocity = new Vector2(speed, speed);
    }

    public void update(){
        position.add(velocity);
        if(position.x + radius > Gdx.graphics.getWidth() || position.x - radius <= 0){
            velocity.scl(-1, 1);
        }

        if(position.y + radius > Gdx.graphics.getHeight() || position.y - radius <= 0){
            velocity.scl(1, -1);
        }
    }

    public void render(ShapeRenderer shapeRenderer){
        shapeRenderer.setColor(Color.GRAY);
        shapeRenderer.circle(position.x, position.y, radius);
    }

    public Vector2 getPosition() {
        return position;
    }
}
