package com.grouter.main;

import com.badlogic.gdx.graphics.Color;
import com.badlogic.gdx.graphics.glutils.ShapeRenderer;
import com.badlogic.gdx.math.Vector2;

import java.util.ArrayList;

public class Tentacle {

    private final float startingAngle = -90f;

    private ArrayList<Segment> segments;
    private boolean ignoreBase;
    private Vector2 target;
    private Vector2 base;

    public Tentacle(float x, float y, int segment_count, float tentacle_length){
        // create segments
        segments = new ArrayList<Segment>();
        segments.add(new Segment(x, y, tentacle_length/(float)segment_count, startingAngle));
        for (int i = 1; i < segment_count; i++){
            segments.add(new Segment(segments.get(i - 1), tentacle_length/(float)segment_count, startingAngle));
        }

        // assign children to all segments (except the last one)
        for (int i = 0; i < segment_count-1; i++){
            segments.get(i).setChild(segments.get(i + 1));
        }

        // creating color gradient on tentacle
        Color color = Color.BLACK;
        for (int i = 0; i < segment_count; i++){
            color.add(0f, 1f / segment_count, 0f, 0f);
            segments.get(i).setColor(color.cpy());
        }

        // tentacle's root
        ignoreBase = false;
        base = new Vector2(x, y);

        target = new Vector2();
    }

    public void removeBase(){
        ignoreBase = true;
    }

    public void update(){
        // last segment will follow target and others their children
        segments.get(segments.size() - 1).update(target.x, target.y);
        for(int i = segments.size() - 2; i >= 0; i--){
            segments.get(i).update();
        }

        // first segment A position will be base(if not ignoring base) and others A position will be their parent B position
        if(!ignoreBase){
            segments.get(0).setA(base);
        }
        for(int i = 1; i < segments.size(); i++){
            segments.get(i).setA(segments.get(i).getParentB());
        }
    }

    public void render(ShapeRenderer shapeRenderer){
        for(int i = 0; i < segments.size(); i++){
            segments.get(i).render(shapeRenderer);
        }
    }

    // GETTERS AND SETTERS


    public boolean isIgnoreBase() {
        return ignoreBase;
    }

    public void setBase(float x, float y){
        base.set(x, y);
    }

    public void setTarget(float x, float y){
        target.set(x, y);
    }

    public void setIgnoreBase(boolean ignoreBase) {
        this.ignoreBase = ignoreBase;
    }
}
