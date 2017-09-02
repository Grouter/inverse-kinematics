package com.grouter.main;

import com.badlogic.gdx.graphics.Color;
import com.badlogic.gdx.graphics.glutils.ShapeRenderer;
import com.badlogic.gdx.math.MathUtils;
import com.badlogic.gdx.math.Vector2;

import static com.grouter.main.Main.SEGMENT_WIDTH;

public class Segment {

    private Vector2 a;
    private Vector2 b;
    private float length;
    private float angle;
    private Segment parent;
    private Segment child;
    private Color color = Color.WHITE;

    public Segment(float x, float y, float length, float angle_from_x){
        a = new Vector2(x, y);
        b = new Vector2();
        this.length = length;
        angle = angle_from_x;
        calcB();
    }

    public Segment(Segment parent, float length, float angle_from_x){
        a = parent.getB();
        b = new Vector2();
        this.length = length;
        angle = angle_from_x;
        this.parent = parent;
        calcB();
    }

    // calculate B position based on A and ANGLE
    public void calcB(){
        float dx = length * MathUtils.cos(angle * MathUtils.PI / 180f);
        float dy = length * MathUtils.sin(angle * MathUtils.PI / 180f);
        b.set(a.x + dx, a.y + dy);
    }

    // set A position and angle to point towards target
    public void follow(float target_x, float target_y){
        Vector2 target = new Vector2(target_x, target_y);
        Vector2 dir = new Vector2(target_x - a.x, target_y - a.y);
        angle = dir.angle();
        dir.setLength(length);
        dir.scl(-1, -1);
        a = target.add(dir);
    }

    public void follow(){
        follow(child.getA().x, child.getA().y);
    }

    public void update(float x, float y){
        follow(x, y);
        calcB();
    }

    public void update(){
        follow();
        calcB();
    }

    public void render(ShapeRenderer shapeRenderer){
        shapeRenderer.setColor(color);
        shapeRenderer.rectLine(a.x, a.y, b.x, b.y, SEGMENT_WIDTH);
    }

    // GETTERS AND SETTERS


    public void setChild(Segment child) {
        this.child = child;
    }

    public void setA(Vector2 a) {
        this.a = a;
        calcB();
    }

    public void setColor(Color color) {
        this.color = color;
    }

    public Vector2 getA() {
        return a;
    }

    public Vector2 getB() {
        return b;
    }

    public Segment getParent() {
        return parent;
    }

    public Vector2 getParentB(){
        return getParent().getB();
    }
}
